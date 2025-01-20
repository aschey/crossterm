use std::{io, time::Duration};

#[cfg(all(feature = "event-stream", not(target_arch = "wasm32")))]
use super::sys::Waker;
use super::InternalEvent;

#[cfg(unix)]
pub(crate) mod unix;
#[cfg(windows)]
pub(crate) mod windows;

#[derive(Default, Clone)]
pub(crate) struct ParseOptions {
    pub(crate) reading_cursor_position: bool,
}

/// An interface for trying to read an `InternalEvent` within an optional `Duration`.
pub(crate) trait EventSource: Sync + Send {
    /// Tries to read an `InternalEvent` within the given duration.
    ///
    /// # Arguments
    ///
    /// * `timeout` - `None` block indefinitely until an event is available, `Some(duration)` blocks
    ///               for the given timeout
    ///
    /// Returns `Ok(None)` if there's no event available and timeout expires.
    fn try_read(
        &mut self,
        timeout: Option<Duration>,
        options: &ParseOptions,
    ) -> io::Result<Option<InternalEvent>>;

    /// Returns a `Waker` allowing to wake/force the `try_read` method to return `Ok(None)`.
    #[cfg(all(feature = "event-stream", not(target_arch = "wasm32")))]
    fn waker(&self) -> Waker;
}
