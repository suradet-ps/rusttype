//! Completed-session history persisted to localStorage.
//!
//! The store is bounded ([`MAX_SESSIONS`]) and versioned via the
//! `rusttype:history:v1` key, so a future sync layer can migrate the schema
//! without guessing at the shape.

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

/// Versioned localStorage key for session history.
const HISTORY_KEY: &str = "rusttype:history:v1";

/// Oldest sessions are dropped once the store grows past this bound.
pub const MAX_SESSIONS: usize = 500;

/// One completed typing session, captured the moment the final character
/// lands.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionRecord {
  /// Wall-clock completion time (Unix epoch, milliseconds).
  pub completed_at_ms: f64,
  /// Words per minute for the session.
  pub wpm: f64,
  /// Accuracy as a fraction (`0.0` to `1.0`).
  pub accuracy: f64,
  /// Total number of wrong keystrokes.
  pub error_count: usize,
  /// Duration in milliseconds from first keystroke to completion.
  pub duration_ms: f64,
  /// Identifier of the snippet that was typed.
  pub snippet_id: String,
  /// Snippet title at completion time; kept even if the snippet is removed.
  pub snippet_title: String,
  /// Top mistyped tokens, kept for drill mode across sessions.
  pub worst_tokens: Vec<(String, usize)>,
}

impl SessionRecord {
  /// Snapshot a finished session from its snippet and final stats.
  pub fn new(
    snippet: &snippets::Snippet,
    stats: &engine::SessionStats,
    completed_at_ms: f64,
  ) -> Self {
    Self {
      completed_at_ms,
      wpm: stats.wpm,
      accuracy: stats.accuracy,
      error_count: stats.error_count,
      duration_ms: stats.duration_ms,
      snippet_id: snippet.id.clone(),
      snippet_title: snippet.title.clone(),
      worst_tokens: stats.worst_tokens.clone(),
    }
  }
}

/// All persisted sessions, oldest first.
///
/// `#[serde(default)]` keeps the schema growable: missing fields fall back
/// to empty values instead of failing the whole load.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct History {
  /// Chronological sessions (oldest first).
  pub sessions: Vec<SessionRecord>,
}

impl History {
  /// Load history from localStorage, falling back to an empty history on
  /// any failure (storage unavailable, corrupt JSON, key absent).
  /// Never panics.
  pub fn load() -> Self {
    let Some(raw) = storage().and_then(|storage| storage.get_item(HISTORY_KEY).ok().flatten())
    else {
      return Self::default();
    };
    serde_json::from_str(&raw).unwrap_or_else(|error| {
      log_failure(&error.to_string());
      Self::default()
    })
  }

  /// Persist history to localStorage. Failures are logged, never panicked.
  pub fn save(&self) {
    let Ok(json) = serde_json::to_string(self) else {
      log_failure("serialization failed");
      return;
    };
    let Some(storage) = storage() else { return };
    if let Err(error) = storage.set_item(HISTORY_KEY, &json) {
      log_failure(&format!("{error:?}"));
    }
  }

  /// Append a session, dropping the oldest records past [`MAX_SESSIONS`].
  pub fn push(&mut self, record: SessionRecord) {
    self.sessions.push(record);
    if self.sessions.len() > MAX_SESSIONS {
      let excess = self.sessions.len() - MAX_SESSIONS;
      self.sessions.drain(..excess);
    }
  }

  /// Total number of stored sessions.
  pub fn total_sessions(&self) -> usize {
    self.sessions.len()
  }

  /// Highest WPM across stored sessions (`0.0` when empty).
  pub fn best_wpm(&self) -> f64 {
    self.sessions.iter().map(|s| s.wpm).fold(0.0, f64::max)
  }

  /// Mean WPM across stored sessions (`0.0` when empty).
  pub fn average_wpm(&self) -> f64 {
    if self.sessions.is_empty() {
      return 0.0;
    }
    self.sessions.iter().map(|s| s.wpm).sum::<f64>() / self.sessions.len() as f64
  }

  /// Mean accuracy across stored sessions (`0.0` when empty).
  pub fn average_accuracy(&self) -> f64 {
    if self.sessions.is_empty() {
      return 0.0;
    }
    self.sessions.iter().map(|s| s.accuracy).sum::<f64>() / self.sessions.len() as f64
  }

  /// The newest `limit` sessions, newest first.
  pub fn recent(&self, limit: usize) -> Vec<&SessionRecord> {
    self.sessions.iter().rev().take(limit).collect()
  }
}

/// Access browser localStorage without panicking.
fn storage() -> Option<web_sys::Storage> {
  web_sys::window()?.local_storage().ok()?
}

/// Log a history failure so corruption is never silent.
fn log_failure(reason: &str) {
  web_sys::console::log_1(&JsValue::from_str(&format!(
    "rusttype: failed to load session history ({reason}); using empty history"
  )));
}

#[cfg(test)]
mod tests {
  use super::*;
  use engine::SessionStats;
  use snippets::{Snippet, SnippetSource};

  fn snippet() -> Snippet {
    Snippet {
      id: "embedded-1".into(),
      source: SnippetSource::Embedded,
      title: "Demo".into(),
      code: "fn main() {}".into(),
    }
  }

  fn stats(wpm: f64, accuracy: f64) -> SessionStats {
    SessionStats {
      wpm,
      accuracy,
      error_count: 1,
      duration_ms: 1000.0,
      worst_tokens: vec![("::".into(), 2)],
    }
  }

  fn record(wpm: f64) -> SessionRecord {
    SessionRecord::new(&snippet(), &stats(wpm, 0.9), 1.0)
  }

  #[test]
  fn record_snapshots_stats_and_snippet() {
    let record = SessionRecord::new(&snippet(), &stats(42.5, 0.875), 1234.0);
    assert_eq!(record.completed_at_ms, 1234.0);
    assert_eq!(record.wpm, 42.5);
    assert_eq!(record.accuracy, 0.875);
    assert_eq!(record.error_count, 1);
    assert_eq!(record.duration_ms, 1000.0);
    assert_eq!(record.snippet_id, "embedded-1");
    assert_eq!(record.snippet_title, "Demo");
    assert_eq!(record.worst_tokens, vec![("::".to_string(), 2)]);
  }

  #[test]
  fn push_trims_oldest_beyond_cap() {
    let mut history = History::default();
    for i in 0..MAX_SESSIONS + 3 {
      history.push(SessionRecord {
        completed_at_ms: i as f64,
        ..record(10.0)
      });
    }
    assert_eq!(history.total_sessions(), MAX_SESSIONS);
    let first = history.sessions.first().expect("non-empty after trimming");
    assert_eq!(first.completed_at_ms, 3.0);
  }

  #[test]
  fn averages_and_best() {
    let mut history = History::default();
    history.push(record(20.0));
    history.push(record(40.0));
    assert_eq!(history.best_wpm(), 40.0);
    assert_eq!(history.average_wpm(), 30.0);
    assert!((history.average_accuracy() - 0.9).abs() < f64::EPSILON);
  }

  #[test]
  fn empty_history_stats_are_zero() {
    let history = History::default();
    assert_eq!(history.best_wpm(), 0.0);
    assert_eq!(history.average_wpm(), 0.0);
    assert_eq!(history.average_accuracy(), 0.0);
    assert_eq!(history.total_sessions(), 0);
    assert!(history.recent(10).is_empty());
  }

  #[test]
  fn recent_returns_newest_first() {
    let mut history = History::default();
    history.push(SessionRecord {
      completed_at_ms: 1.0,
      ..record(10.0)
    });
    history.push(SessionRecord {
      completed_at_ms: 2.0,
      ..record(20.0)
    });
    let recent = history.recent(1);
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].completed_at_ms, 2.0);
  }

  #[test]
  fn serde_round_trip_preserves_records() {
    let mut history = History::default();
    history.push(record(33.3));
    let json = serde_json::to_string(&history).expect("history serializes");
    let restored: History = serde_json::from_str(&json).expect("history deserializes");
    assert_eq!(restored, history);
  }

  #[test]
  fn missing_fields_fall_back_to_defaults() {
    let restored: History = serde_json::from_str("{}").expect("empty object is accepted");
    assert!(restored.sessions.is_empty());
  }
}
