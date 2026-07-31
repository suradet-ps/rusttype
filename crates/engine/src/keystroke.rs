/// A single keystroke event recorded during a typing session.
#[derive(Debug, Clone)]
pub struct KeystrokeEvent {
  /// The character the target expected at this position.
  pub expected: char,
  /// The character the user actually typed.
  pub actual: char,
  /// Whether the keystroke matched the target.
  pub correct: bool,
  /// Timestamp in milliseconds (from performance.now() or equivalent).
  pub at_ms: f64,
}

/// The result of processing a single keystroke.
#[derive(Debug)]
pub enum KeyResult {
  /// The keystroke was correct; cursor has advanced to the given position.
  Correct { cursor: usize },
  /// The keystroke was wrong; cursor has not moved.
  Wrong { expected: char, actual: char },
  /// The entire target has been typed correctly; session is complete.
  Completed { stats: SessionStats },
}

use crate::stats::SessionStats;
