//! Event trait definitions.

use crate::event::{EventAction, EventKind};

/// Common interface for all event handlers.
pub trait EventLike {
    /// Returns event kind.
    fn kind(&self) -> &EventKind;

    /// Returns associated action.
    fn action(&self) -> &EventAction;
}
