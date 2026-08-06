//! Error-event construction used by parser rules.

use crate::cst::{ErrorKind, Event};

pub(crate) fn error_at(events: &mut Vec<Event>, token_index: Option<usize>, kind: ErrorKind) {
    events.push(Event::Error {
        token_index: token_index.map(|index| index as u32),
        kind,
    });
}
