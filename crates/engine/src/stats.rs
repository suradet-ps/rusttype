/// Performance statistics for a completed typing session.
#[derive(Debug, Clone)]
pub struct SessionStats {
  /// Words per minute: `(correct_chars / 5) / minutes_elapsed`.
  pub wpm: f64,
  /// Accuracy: `correct_keystrokes / total_keystrokes`.
  pub accuracy: f64,
  /// Total number of wrong keystrokes.
  pub error_count: usize,
  /// Duration in milliseconds from first keystroke to completion.
  pub duration_ms: f64,
  /// Top mistyped substrings and their error counts (for drill mode).
  pub worst_tokens: Vec<(String, usize)>,
}

impl SessionStats {
  /// Compute stats from a list of keystroke events.
  ///
  /// Returns zeroed stats if the event list is empty.
  pub fn compute(keystrokes: &[crate::keystroke::KeystrokeEvent], error_count: usize) -> Self {
    if keystrokes.is_empty() {
      return Self {
        wpm: 0.0,
        accuracy: 0.0,
        error_count: 0,
        duration_ms: 0.0,
        worst_tokens: Vec::new(),
      };
    }

    let first = keystrokes.first().expect("non-empty");
    let last = keystrokes.last().expect("non-empty");
    let duration_ms = last.at_ms - first.at_ms;

    let correct_count = keystrokes.iter().filter(|k| k.correct).count();
    let total = keystrokes.len();

    let minutes = duration_ms / 60_000.0;
    let wpm = if minutes > 0.0 {
      (correct_count as f64 / 5.0) / minutes
    } else {
      0.0
    };

    let accuracy = if total > 0 {
      correct_count as f64 / total as f64
    } else {
      0.0
    };

    let worst_tokens = compute_worst_tokens(keystrokes);

    Self {
      wpm,
      accuracy,
      error_count,
      duration_ms,
      worst_tokens,
    }
  }
}

/// Analyze keystrokes to find the most-mistyped substrings.
fn compute_worst_tokens(keystrokes: &[crate::keystroke::KeystrokeEvent]) -> Vec<(String, usize)> {
  use std::collections::HashMap;

  let mut error_positions: Vec<usize> = Vec::new();
  for (i, k) in keystrokes.iter().enumerate() {
    if !k.correct {
      error_positions.push(i);
    }
  }

  if error_positions.is_empty() {
    return Vec::new();
  }

  let full: String = keystrokes.iter().map(|k| k.expected).collect();
  let mut token_errors: HashMap<String, usize> = HashMap::new();

  for &pos in &error_positions {
    for window_size in 2..=3 {
      let start = pos.saturating_sub(window_size / 2);
      let end = (start + window_size).min(full.len());
      if end > start {
        let substring: String = full.chars().skip(start).take(end - start).collect();
        *token_errors.entry(substring).or_insert(0) += 1;
      }
    }
  }

  let mut tokens: Vec<(String, usize)> = token_errors.into_iter().collect();
  tokens.sort_by_key(|b| std::cmp::Reverse(b.1));
  tokens.truncate(5);
  tokens
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::keystroke::KeystrokeEvent;

  fn make_event(expected: char, actual: char, at_ms: f64) -> KeystrokeEvent {
    KeystrokeEvent {
      expected,
      actual,
      correct: expected == actual,
      at_ms,
    }
  }

  #[test]
  fn stats_empty_events() {
    let stats = SessionStats::compute(&[], 0);
    assert_eq!(stats.wpm, 0.0);
    assert_eq!(stats.accuracy, 0.0);
    assert_eq!(stats.error_count, 0);
    assert_eq!(stats.duration_ms, 0.0);
  }

  #[test]
  fn stats_all_correct() {
    let events: Vec<KeystrokeEvent> = "hello"
      .chars()
      .enumerate()
      .map(|(i, c)| make_event(c, c, (i as f64 + 1.0) * 100.0))
      .collect();

    let stats = SessionStats::compute(&events, 0);
    assert_eq!(stats.error_count, 0);
    assert!((stats.accuracy - 1.0).abs() < f64::EPSILON);
    assert!(stats.wpm > 0.0);
    assert!(stats.duration_ms > 0.0);
  }

  #[test]
  fn stats_with_errors() {
    let mut events = Vec::new();
    for (i, (expected, actual)) in "hXllo".chars().zip("hello".chars()).enumerate() {
      events.push(make_event(expected, actual, (i as f64 + 1.0) * 100.0));
    }

    let stats = SessionStats::compute(&events, 1);
    assert_eq!(stats.error_count, 1);
    assert!((stats.accuracy - 0.8).abs() < f64::EPSILON);
  }

  #[test]
  fn stats_zero_duration_gives_zero_wpm() {
    let events = vec![make_event('a', 'a', 100.0)];

    let stats = SessionStats::compute(&events, 0);
    assert_eq!(stats.wpm, 0.0);
  }
}
