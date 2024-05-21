use std::{
    cell::{Cell, OnceCell},
    io,
    sync::{Mutex, OnceLock},
    task::{Context, Poll},
};

use futures::{channel::mpsc, StreamExt};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlElement;

use crate::terminal::WindowSize;

thread_local! {
    static TERMINAL: OnceCell<xterm_js_rs::Terminal> = OnceCell::new();
}

static DATA_CHANNEL: OnceLock<Mutex<mpsc::Receiver<String>>> = OnceLock::new();

pub(crate) fn with_terminal<F, T>(f: F) -> T
where
    F: FnOnce(&xterm_js_rs::Terminal) -> T,
{
    TERMINAL.with(|t| f(t.get().unwrap()))
}

pub fn init_terminal(options: &xterm_js_rs::TerminalOptions, parent: HtmlElement) {
    TERMINAL.with(|t| {
        let (mut tx, rx) = mpsc::channel(32);
        let mut tx_ = tx.clone();
        let terminal = xterm_js_rs::Terminal::new(options);

        let callback = Closure::wrap(Box::new(move |e: xterm_js_rs::Event| {
            tx_.try_send(e.as_string().unwrap()).ok();
        }) as Box<dyn FnMut(_)>);
        terminal.on_data(callback.as_ref().unchecked_ref());
        callback.forget();

        let callback = Closure::wrap(Box::new(move |e: xterm_js_rs::Event| {
            tx.try_send(e.as_string().unwrap()).ok();
        }) as Box<dyn FnMut(_)>);
        terminal.on_binary(callback.as_ref().unchecked_ref());
        callback.forget();

        DATA_CHANNEL.set(Mutex::new(rx)).unwrap();

        terminal.open(parent);
        terminal.focus();
        if t.set(terminal).is_err() {
            panic!();
        }
    });
}

pub(crate) fn poll_next_event(cx: &mut Context<'_>) -> Poll<Option<String>> {
    DATA_CHANNEL
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .poll_next_unpin(cx)
}

pub(crate) fn is_raw_mode_enabled() -> bool {
    true
}

pub(crate) fn window_size() -> io::Result<WindowSize> {
    Ok(super::wasm::with_terminal(|t| WindowSize {
        rows: t.get_rows() as u16,
        columns: t.get_cols() as u16,
        width: t.get_element().client_width() as u16,
        height: t.get_element().client_height() as u16,
    }))
}

pub(crate) fn size() -> io::Result<(u16, u16)> {
    window_size().map(|s| (s.columns, s.rows))
}

pub(crate) fn disable_raw_mode() -> io::Result<()> {
    Ok(())
}

pub(crate) fn enable_raw_mode() -> io::Result<()> {
    Ok(())
}

#[cfg(feature = "events")]
pub fn supports_keyboard_enhancement() -> io::Result<bool> {
    Ok(false)
}

#[derive(Default)]
pub struct TerminalHandle {
    buffer: Cell<Vec<u8>>,
}

impl TerminalHandle {
    #[inline]
    fn flush_immutable(&self) -> io::Result<()> {
        // Can't call `self.buffer.flush()` here but since that's just a Vec,
        // it's probably fine.

        let s = String::from_utf8(self.buffer.replace(Vec::new()))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        with_terminal(|t| t.write(&s));

        Ok(())
    }
}

impl io::Write for TerminalHandle {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.get_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.get_mut().flush()?;
        self.flush_immutable()
    }
}
