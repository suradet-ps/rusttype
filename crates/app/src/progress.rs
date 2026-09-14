//! Challenge progress persisted to localStorage: best result per level.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

/// Versioned localStorage key for challenge progress.
const PROGRESS_KEY: &str = "rusttype:progress:v1";

/// Best attempt recorded for one challenge level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelBest {
  /// Stars earned (`1..=3`).
  pub stars: u8,
  /// Words per minute of the best attempt.
  pub wpm: f64,
  /// Accuracy (fraction) of the best attempt.
  pub accuracy: f64,
  /// Wall-clock time of the best attempt (Unix epoch, milliseconds).
  pub completed_at_ms: f64,
}

/// Challenge progress keyed by snippet id.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Progress {
  /// Best result per challenge id.
  pub levels: BTreeMap<String, LevelBest>,
}

impl Progress {
  /// Load progress from localStorage, falling back to empty progress on any
  /// failure (storage unavailable, corrupt JSON, key absent). Never panics.
  pub fn load() -> Self {
    let Some(raw) = storage().and_then(|storage| storage.get_item(PROGRESS_KEY).ok().flatten())
    else {
      return Self::default();
    };
    serde_json::from_str(&raw).unwrap_or_else(|error| {
      log_failure(&error.to_string());
      Self::default()
    })
  }

  /// Persist progress to localStorage. Failures are logged, never panicked.
  pub fn save(&self) {
    let Ok(json) = serde_json::to_string(self) else {
      log_failure("serialization failed");
      return;
    };
    let Some(storage) = storage() else { return };
    if let Err(error) = storage.set_item(PROGRESS_KEY, &json) {
      log_failure(&format!("{error:?}"));
    }
  }

  /// Record an attempt, keeping it only when it improves on the stored best.
  ///
  /// Returns `true` when the stored best changed.
  pub fn record(
    &mut self,
    level_id: &str,
    stars: u8,
    wpm: f64,
    accuracy: f64,
    completed_at_ms: f64,
  ) -> bool {
    let candidate = LevelBest {
      stars,
      wpm,
      accuracy,
      completed_at_ms,
    };
    match self.levels.get(level_id) {
      Some(best) if !is_better(&candidate, best) => false,
      _ => {
        self.levels.insert(level_id.to_string(), candidate);
        true
      }
    }
  }

  /// Best result for a level, if it was ever attempted.
  pub fn best(&self, level_id: &str) -> Option<&LevelBest> {
    self.levels.get(level_id)
  }

  /// Total stars earned across every level.
  pub fn total_stars(&self) -> u32 {
    self.levels.values().map(|best| best.stars as u32).sum()
  }
}

/// Star label for `stars` out of three, e.g. `★★☆`.
pub fn stars_label(stars: u8) -> String {
  let filled = stars.min(3) as usize;
  "★".repeat(filled) + &"☆".repeat(3 - filled)
}

/// Stars earned across a set of levels.
pub fn stars_for_levels(progress: &Progress, levels: &[&snippets::Challenge]) -> u32 {
  levels
    .iter()
    .map(|level| progress.best(level.id).map_or(0, |best| best.stars as u32))
    .sum()
}

/// An attempt improves when it earns more stars, or the same stars at a
/// higher WPM.
fn is_better(candidate: &LevelBest, best: &LevelBest) -> bool {
  candidate.stars > best.stars || (candidate.stars == best.stars && candidate.wpm > best.wpm)
}

/// Access browser localStorage without panicking.
fn storage() -> Option<web_sys::Storage> {
  web_sys::window()?.local_storage().ok()?
}

/// Log a progress failure so corruption is never silent.
fn log_failure(reason: &str) {
  web_sys::console::log_1(&JsValue::from_str(&format!(
    "rusttype: failed to load challenge progress ({reason}); using empty progress"
  )));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn record_keeps_the_best_attempt() {
    let mut progress = Progress::default();
    assert!(progress.record("challenge-rg-01", 2, 30.0, 0.98, 1.0));
    assert!(
      !progress.record("challenge-rg-01", 2, 25.0, 0.99, 2.0),
      "worse WPM"
    );
    assert!(
      progress.record("challenge-rg-01", 3, 20.0, 0.98, 3.0),
      "more stars"
    );
    assert!(
      !progress.record("challenge-rg-01", 2, 60.0, 0.98, 4.0),
      "fewer stars"
    );
    let best = progress.best("challenge-rg-01").expect("recorded");
    assert_eq!(best.stars, 3);
    assert_eq!(best.wpm, 20.0);
    assert_eq!(best.completed_at_ms, 3.0);
  }

  #[test]
  fn same_stars_higher_wpm_wins() {
    let mut progress = Progress::default();
    progress.record("challenge-rg-02", 2, 30.0, 0.98, 1.0);
    assert!(progress.record("challenge-rg-02", 2, 31.0, 0.97, 2.0));
    assert_eq!(
      progress.best("challenge-rg-02").expect("recorded").wpm,
      31.0
    );
  }

  #[test]
  fn total_stars_sums_levels() {
    let mut progress = Progress::default();
    progress.record("a", 1, 10.0, 0.9, 1.0);
    progress.record("b", 3, 40.0, 1.0, 2.0);
    assert_eq!(progress.total_stars(), 4);
    assert!(Progress::default().best("a").is_none());
  }

  #[test]
  fn serde_round_trip_preserves_progress() {
    let mut progress = Progress::default();
    progress.record("challenge-rg-01", 3, 42.5, 0.99, 1234.0);
    let json = serde_json::to_string(&progress).expect("progress serializes");
    let restored: Progress = serde_json::from_str(&json).expect("progress deserializes");
    assert_eq!(restored, progress);
  }

  #[test]
  fn missing_fields_fall_back_to_defaults() {
    let restored: Progress = serde_json::from_str("{}").expect("empty object is accepted");
    assert!(restored.levels.is_empty());
  }

  #[test]
  fn stars_label_renders_three_slots() {
    assert_eq!(stars_label(0), "☆☆☆");
    assert_eq!(stars_label(2), "★★☆");
    assert_eq!(stars_label(5), "★★★");
  }

  #[test]
  fn stars_for_levels_sums_the_progress() {
    use snippets::Challenge;

    fn level(id: &'static str) -> Challenge {
      Challenge {
        id,
        title: "Title",
        project: "demo",
        source_path: "src/lib.rs",
        license: "MIT",
        level: 1,
        accuracy_goal: 0.97,
        wpm_goal: 30.0,
        code: "fn main() {}\n",
      }
    }

    let levels = [level("a"), level("b")];
    let refs: Vec<&Challenge> = levels.iter().collect();
    let mut progress = Progress::default();
    progress.record("a", 3, 40.0, 1.0, 1.0);
    progress.record("b", 2, 30.0, 0.98, 2.0);
    assert_eq!(stars_for_levels(&progress, &refs), 5);
    assert_eq!(stars_for_levels(&Progress::default(), &refs), 0);
  }
}
