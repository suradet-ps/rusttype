use crate::keystroke::{KeyResult, KeystrokeEvent};
use crate::stats::SessionStats;

/// Core typing state machine enforcing strict-mode rules.
///
/// In strict mode, the cursor only advances on correct keystrokes.
/// Newlines and indentation whitespace are real target characters.
/// Timestamps are `f64` milliseconds — the caller provides them via `process_key_at`.
#[derive(Debug, Clone)]
pub struct TypingState {
  /// The target characters to type.
  pub target: Vec<char>,
  /// Current cursor position (index into `target`).
  pub cursor: usize,
  /// Total number of wrong keystrokes (not distinct wrong characters).
  pub error_count: usize,
  /// Time of the first keystroke (ms).
  pub started_at_ms: Option<f64>,
  /// Time of the last correct keystroke that completed the target (ms).
  pub finished_at_ms: Option<f64>,
  /// Log of every keystroke event.
  pub keystrokes: Vec<KeystrokeEvent>,
}

impl TypingState {
  /// Create a new typing state for the given target string.
  ///
  /// Carriage returns are stripped: they are untypeable (Enter produces `\n`)
  /// and appear in CRLF content pasted on Windows, which would softlock
  /// strict mode at every newline.
  pub fn new(target: &str) -> Self {
    Self {
      target: target.chars().filter(|c| *c != '\r').collect(),
      cursor: 0,
      error_count: 0,
      started_at_ms: None,
      finished_at_ms: None,
      keystrokes: Vec::new(),
    }
  }

  /// Process a single keystroke at the given timestamp (ms), returning the result.
  pub fn process_key_at(&mut self, actual: char, now_ms: f64) -> KeyResult {
    if self.started_at_ms.is_none() {
      self.started_at_ms = Some(now_ms);
    }

    if self.cursor >= self.target.len() {
      let stats = self.finalize(now_ms);
      return KeyResult::Completed { stats };
    }

    let expected = self.target[self.cursor];
    let correct = expected == actual;

    self.keystrokes.push(KeystrokeEvent {
      expected,
      actual,
      correct,
      at_ms: now_ms,
    });

    if correct {
      self.cursor += 1;

      // Auto-indent: after a correct newline, skip leading whitespace
      // on the next line — IDE-style. No keystrokes recorded for skipped
      // indentation so WPM / accuracy stay honest.
      if expected == '\n' {
        while self.cursor < self.target.len() && matches!(self.target[self.cursor], ' ' | '\t') {
          self.cursor += 1;
        }
      }

      if self.cursor >= self.target.len() {
        let stats = self.finalize(now_ms);
        KeyResult::Completed { stats }
      } else {
        KeyResult::Correct {
          cursor: self.cursor,
        }
      }
    } else {
      self.error_count += 1;
      KeyResult::Wrong { expected, actual }
    }
  }

  /// Whether the target has been fully typed.
  pub fn is_complete(&self) -> bool {
    self.cursor >= self.target.len()
  }

  /// The current target as a string.
  pub fn target_str(&self) -> String {
    self.target.iter().collect()
  }

  fn finalize(&mut self, now_ms: f64) -> SessionStats {
    self.finished_at_ms = Some(now_ms);
    SessionStats::compute(&self.keystrokes, self.error_count)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_state_starts_at_zero() {
    let state = TypingState::new("ab");
    assert_eq!(state.cursor, 0);
    assert_eq!(state.error_count, 0);
    assert!(!state.is_complete());
  }

  #[test]
  fn crlf_target_normalizes_to_lf() {
    let mut state = TypingState::new("a\r\nb");
    assert_eq!(state.target_str(), "a\nb");
    state.process_key_at('a', 100.0);
    let result = state.process_key_at('\n', 200.0);
    assert!(matches!(result, KeyResult::Correct { cursor: 2 }));
    let result = state.process_key_at('b', 300.0);
    assert!(matches!(result, KeyResult::Completed { .. }));
  }

  #[test]
  fn correct_key_advances_cursor() {
    let mut state = TypingState::new("ab");
    let result = state.process_key_at('a', 100.0);
    assert!(matches!(result, KeyResult::Correct { cursor: 1 }));
    assert_eq!(state.cursor, 1);
    assert!(!state.is_complete());
  }

  #[test]
  fn wrong_key_does_not_advance_cursor() {
    let mut state = TypingState::new("ab");
    let result = state.process_key_at('x', 100.0);
    assert!(matches!(result, KeyResult::Wrong { .. }));
    assert_eq!(state.cursor, 0);
    assert_eq!(state.error_count, 1);
  }

  #[test]
  fn wrong_key_counts_each_attempt() {
    let mut state = TypingState::new("ab");
    state.process_key_at('x', 100.0);
    state.process_key_at('y', 200.0);
    state.process_key_at('z', 300.0);
    assert_eq!(state.error_count, 3);
    assert_eq!(state.cursor, 0);
  }

  #[test]
  fn completion_returns_stats() {
    let mut state = TypingState::new("ab");
    state.process_key_at('a', 100.0);
    let result = state.process_key_at('b', 200.0);
    assert!(matches!(result, KeyResult::Completed { .. }));
    assert!(state.is_complete());
  }

  #[test]
  fn empty_target_completes_immediately() {
    let mut state = TypingState::new("");
    let result = state.process_key_at('a', 0.0);
    assert!(matches!(result, KeyResult::Completed { .. }));
    assert!(state.is_complete());
  }

  #[test]
  fn single_char_target() {
    let mut state = TypingState::new("x");
    let result = state.process_key_at('x', 100.0);
    assert!(matches!(result, KeyResult::Completed { .. }));
  }

  #[test]
  fn target_str_returns_string() {
    let state = TypingState::new("hello");
    assert_eq!(state.target_str(), "hello");
  }

  #[test]
  fn whitespace_and_newlines_are_target_chars() {
    let mut state = TypingState::new("a\tb\nc");
    state.process_key_at('a', 100.0);
    let result = state.process_key_at('\t', 200.0);
    assert!(matches!(result, KeyResult::Correct { cursor: 2 }));
    state.process_key_at('b', 300.0);
    let result = state.process_key_at('\n', 400.0);
    assert!(matches!(result, KeyResult::Correct { cursor: 4 }));
    let result = state.process_key_at('c', 500.0);
    assert!(matches!(result, KeyResult::Completed { .. }));
  }

  #[test]
  fn mixed_correct_and_wrong() {
    let mut state = TypingState::new("abc");
    state.process_key_at('a', 100.0);
    state.process_key_at('x', 200.0);
    state.process_key_at('b', 300.0);
    state.process_key_at('c', 400.0);
    assert_eq!(state.error_count, 1);
    assert!(state.is_complete());
  }

  #[test]
  fn started_at_set_on_first_key() {
    let mut state = TypingState::new("a");
    assert!(state.started_at_ms.is_none());
    state.process_key_at('a', 100.0);
    assert!((state.started_at_ms.unwrap() - 100.0).abs() < f64::EPSILON);
  }

  #[test]
  fn auto_indent_skips_spaces_after_newline() {
    let mut state = TypingState::new("a\n    b");
    state.process_key_at('a', 100.0);
    let result = state.process_key_at('\n', 200.0);
    assert!(matches!(result, KeyResult::Correct { cursor: 6 }));
    assert_eq!(state.cursor, 6);
    assert_eq!(state.keystrokes.len(), 2);
  }

  #[test]
  fn auto_indent_skips_tabs_after_newline() {
    let mut state = TypingState::new("a\n\t\tb");
    state.process_key_at('a', 100.0);
    let result = state.process_key_at('\n', 200.0);
    assert!(matches!(result, KeyResult::Correct { cursor: 4 }));
  }

  #[test]
  fn auto_indent_stops_at_next_newline() {
    let mut state = TypingState::new("a\n  \nb");
    state.process_key_at('a', 100.0);
    let result = state.process_key_at('\n', 200.0);
    assert!(matches!(result, KeyResult::Correct { cursor: 4 }));
  }

  #[test]
  fn auto_indent_completes_on_trailing_whitespace() {
    let mut state = TypingState::new("a\n  ");
    state.process_key_at('a', 100.0);
    let result = state.process_key_at('\n', 200.0);
    assert!(matches!(result, KeyResult::Completed { .. }));
  }

  #[test]
  fn auto_indent_does_not_affect_accuracy() {
    let mut state = TypingState::new("a\n    b");
    state.process_key_at('a', 100.0);
    state.process_key_at('\n', 200.0);
    let result = state.process_key_at('b', 300.0);
    if let KeyResult::Completed { stats } = result {
      assert!((stats.accuracy - 1.0).abs() < f64::EPSILON);
    } else {
      panic!("expected Completed");
    }
  }
}
