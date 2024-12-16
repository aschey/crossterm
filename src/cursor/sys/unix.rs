use std::{
    io::{self, Error, ErrorKind},
    time::{Duration, Instant},
};

use crate::{
    event::{filter::CursorPositionFilter, poll_internal, read_internal, InternalEvent},
    terminal::{
        disable_raw_mode, enable_raw_mode,
        sys::{file_descriptor::tty_fd_out, is_raw_mode_enabled},
    },
};

/// Returns the cursor position (column, row).
///
/// The top left cell is represented as `(0, 0)`.
///
/// On unix systems, this function will block and possibly time out while
/// [`crossterm::event::read`](crate::event::read) or [`crossterm::event::poll`](crate::event::poll) are being called.
pub fn position() -> io::Result<(u16, u16)> {
    if is_raw_mode_enabled() {
        read_position_raw()
    } else {
        read_position()
    }
}

fn read_position() -> io::Result<(u16, u16)> {
    enable_raw_mode()?;
    let pos = read_position_raw();
    disable_raw_mode()?;
    pos
}

fn read_position_raw() -> io::Result<(u16, u16)> {
    // Use `ESC [ 6 n` to and retrieve the cursor position.
    let stdout = tty_fd_out()?;
    stdout.write(b"\x1B[6n")?;

    let poll_timeout = Duration::from_millis(2000);
    let poll_start = Instant::now();
    loop {
        match poll_internal(Some(poll_timeout), &CursorPositionFilter) {
            Ok(true) => {
                if let Ok(InternalEvent::CursorPosition(x, y)) =
                    read_internal(&CursorPositionFilter)
                {
                    return Ok((x, y));
                }
            }
            Ok(false) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "The cursor position could not be read within a normal duration",
                ));
            }
            Err(e) if Instant::now() - poll_start > poll_timeout => {
                return Err(io::Error::new(
                    e.kind(),
                    format!("Error reading cursor position: {e:?}"),
                ));
            }
            Err(_) => {}
        }
    }
}
