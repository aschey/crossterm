use std::{
    io,
    task::{ready, Poll},
};

use futures_core::Stream;

use crate::{
    event::{sys::unix::parse::parse_event, InternalEvent},
    terminal::sys::poll_next_event,
};

use super::Event;

#[derive(Default)]
pub struct EventStream;

impl Stream for EventStream {
    type Item = io::Result<Event>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        loop {
            if let Some(event) = ready!(poll_next_event(cx)) {
                match parse_event(event.as_bytes(), false) {
                    Ok(Some(InternalEvent::Event(e))) => {
                        return Poll::Ready(Some(Ok(e)));
                    }
                    Err(e) => {
                        return Poll::Ready(Some(Err(e)));
                    }
                    _ => {
                        continue;
                    }
                }
            } else {
                return Poll::Pending;
            }
        }
    }
}
