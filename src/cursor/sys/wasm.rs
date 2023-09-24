use std::io;

use crate::terminal::sys::with_terminal;

pub fn position() -> io::Result<(u16, u16)> {
    Ok(with_terminal(|t| {
        let active = t.get_buffer().get_active();
        (active.get_cursor_x() as u16, active.get_cursor_y() as u16)
    }))
}
