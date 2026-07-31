//! Typing state machine, WPM/accuracy calc, keystroke event log.

mod keystroke;
mod state;
mod stats;

pub use keystroke::{KeyResult, KeystrokeEvent};
pub use state::TypingState;
pub use stats::SessionStats;
