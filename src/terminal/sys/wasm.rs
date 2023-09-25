use std::io;

use crate::terminal::WindowSize;

pub(crate) fn is_raw_mode_enabled() -> bool {
    true
}

pub(crate) fn window_size() -> io::Result<WindowSize> {
    Ok(WindowSize {
        rows: 0,
        columns: 0,
        width: 0,
        height: 0,
    })
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
