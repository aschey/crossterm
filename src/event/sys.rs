#[cfg(all(windows, feature = "event-stream"))]
pub(crate) use windows::waker::Waker;

#[cfg(any(unix, target_arch = "wasm32"))]
pub(crate) mod unix;
#[cfg(windows)]
pub(crate) mod windows;
