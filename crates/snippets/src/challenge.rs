//! Challenge levels: real Rust code from real projects, with goals.
//!
//! Levels are embedded at compile time - no network and no runtime manifest.
//! To add a level, drop the excerpt under `challenges/<project>/` and add one
//! entry to [`CHALLENGES`]. See `challenges/README.md` for the excerpt rules.

use crate::model::{Snippet, SnippetSource};

/// Minimum accuracy (fraction) for the second star.
pub const TWO_STAR_ACCURACY: f64 = 0.97;

/// One curated challenge level.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Challenge {
  /// Snippet id, e.g. `challenge-rg-01`.
  pub id: &'static str,
  /// Display title.
  pub title: &'static str,
  /// Source project, e.g. `ripgrep`.
  pub project: &'static str,
  /// Path of the excerpt inside the source project.
  pub source_path: &'static str,
  /// License of the source project.
  pub license: &'static str,
  /// Ordering within the challenge list.
  pub level: u32,
  /// Accuracy goal (fraction) for the second star.
  pub accuracy_goal: f64,
  /// WPM goal for the third star.
  pub wpm_goal: f64,
  /// The excerpt itself.
  pub code: &'static str,
}

impl Challenge {
  /// Build the practice snippet for this level.
  pub fn snippet(&self) -> Snippet {
    Snippet {
      id: self.id.to_string(),
      source: SnippetSource::Challenge,
      title: self.title.to_string(),
      code: self.code.trim_end().to_string(),
    }
  }

  /// Star rating for a finished attempt.
  ///
  /// One star for finishing, two for the accuracy goal, three for also
  /// reaching the WPM goal.
  pub fn stars(&self, wpm: f64, accuracy: f64) -> u8 {
    if accuracy < self.accuracy_goal {
      return 1;
    }
    if wpm >= self.wpm_goal { 3 } else { 2 }
  }
}

/// Every embedded level, in level order.
static CHALLENGES: &[Challenge] = &[
  Challenge {
    id: "challenge-rg-01",
    title: "Byte escapes",
    project: "ripgrep",
    source_path: "crates/cli/src/escape.rs",
    license: "MIT",
    level: 1,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 25.0,
    code: include_str!("../../../challenges/ripgrep/01-byte-escapes.rs"),
  },
  Challenge {
    id: "challenge-rg-02",
    title: "Literal globs",
    project: "ripgrep",
    source_path: "crates/globset/src/glob.rs",
    license: "MIT",
    level: 2,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 28.0,
    code: include_str!("../../../challenges/ripgrep/02-literal-globs.rs"),
  },
  Challenge {
    id: "challenge-rg-03",
    title: "Override matching",
    project: "ripgrep",
    source_path: "crates/ignore/src/overrides.rs",
    license: "MIT",
    level: 3,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 30.0,
    code: include_str!("../../../challenges/ripgrep/03-override-matching.rs"),
  },
  Challenge {
    id: "challenge-rg-04",
    title: "Preceding lines",
    project: "ripgrep",
    source_path: "crates/searcher/src/lines.rs",
    license: "MIT",
    level: 4,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 33.0,
    code: include_str!("../../../challenges/ripgrep/04-preceding-lines.rs"),
  },
  Challenge {
    id: "challenge-rg-05",
    title: "Glob extensions",
    project: "ripgrep",
    source_path: "crates/globset/src/glob.rs",
    license: "MIT",
    level: 5,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 35.0,
    code: include_str!("../../../challenges/ripgrep/05-glob-extensions.rs"),
  },
  Challenge {
    id: "challenge-rg-06",
    title: "Match offsets",
    project: "ripgrep",
    source_path: "crates/matcher/src/lib.rs",
    license: "MIT",
    level: 6,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 38.0,
    code: include_str!("../../../challenges/ripgrep/06-match-offsets.rs"),
  },
  Challenge {
    id: "challenge-serde-01",
    title: "Size hints",
    project: "serde",
    source_path: "serde_core/src/private/size_hint.rs",
    license: "MIT OR Apache-2.0",
    level: 1,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 26.0,
    code: include_str!("../../../challenges/serde/01-size-hints.rs"),
  },
  Challenge {
    id: "challenge-serde-02",
    title: "Ignored fields",
    project: "serde",
    source_path: "serde_core/src/de/ignored_any.rs",
    license: "MIT OR Apache-2.0",
    level: 2,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 29.0,
    code: include_str!("../../../challenges/serde/02-ignored-fields.rs"),
  },
  Challenge {
    id: "challenge-serde-03",
    title: "Unit deserializer",
    project: "serde",
    source_path: "serde_core/src/de/value.rs",
    license: "MIT OR Apache-2.0",
    level: 3,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 31.0,
    code: include_str!("../../../challenges/serde/03-unit-deserializer.rs"),
  },
  Challenge {
    id: "challenge-serde-04",
    title: "Derive bounds",
    project: "serde",
    source_path: "serde_derive/src/bound.rs",
    license: "MIT OR Apache-2.0",
    level: 4,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 34.0,
    code: include_str!("../../../challenges/serde/04-derive-bounds.rs"),
  },
  Challenge {
    id: "challenge-serde-05",
    title: "Deserialize hints",
    project: "serde",
    source_path: "serde_core/src/de/mod.rs",
    license: "MIT OR Apache-2.0",
    level: 5,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 36.0,
    code: include_str!("../../../challenges/serde/05-deserialize-hints.rs"),
  },
  Challenge {
    id: "challenge-serde-06",
    title: "Content buffer",
    project: "serde",
    source_path: "serde_core/src/private/content.rs",
    license: "MIT OR Apache-2.0",
    level: 6,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 39.0,
    code: include_str!("../../../challenges/serde/06-content-buffer.rs"),
  },
  Challenge {
    id: "challenge-hb-01",
    title: "Bit mask",
    project: "hashbrown",
    source_path: "src/control/bitmask.rs",
    license: "MIT OR Apache-2.0",
    level: 1,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 27.0,
    code: include_str!("../../../challenges/hashbrown/01-bit-mask.rs"),
  },
  Challenge {
    id: "challenge-hb-02",
    title: "Branch hints",
    project: "hashbrown",
    source_path: "src/util.rs",
    license: "MIT OR Apache-2.0",
    level: 2,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 30.0,
    code: include_str!("../../../challenges/hashbrown/02-branch-hints.rs"),
  },
  Challenge {
    id: "challenge-hb-03",
    title: "Control tags",
    project: "hashbrown",
    source_path: "src/control/tag.rs",
    license: "MIT OR Apache-2.0",
    level: 3,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 32.0,
    code: include_str!("../../../challenges/hashbrown/03-control-tags.rs"),
  },
  Challenge {
    id: "challenge-hb-04",
    title: "Scope guard",
    project: "hashbrown",
    source_path: "src/scopeguard.rs",
    license: "MIT OR Apache-2.0",
    level: 4,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 35.0,
    code: include_str!("../../../challenges/hashbrown/04-scope-guard.rs"),
  },
  Challenge {
    id: "challenge-hb-05",
    title: "Hash builder",
    project: "hashbrown",
    source_path: "src/hasher.rs",
    license: "MIT OR Apache-2.0",
    level: 5,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 37.0,
    code: include_str!("../../../challenges/hashbrown/05-hash-builder.rs"),
  },
  Challenge {
    id: "challenge-hb-06",
    title: "Group matches",
    project: "hashbrown",
    source_path: "src/control/group/generic.rs",
    license: "MIT OR Apache-2.0",
    level: 6,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 40.0,
    code: include_str!("../../../challenges/hashbrown/06-group-matches.rs"),
  },
  Challenge {
    id: "challenge-pt-01",
    title: "No-shrink macro",
    project: "proptest",
    source_path: "proptest/src/strategy/just.rs",
    license: "MIT OR Apache-2.0",
    level: 1,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 28.0,
    code: include_str!("../../../challenges/proptest/01-no-shrink-macro.rs"),
  },
  Challenge {
    id: "challenge-pt-02",
    title: "Bit set trait",
    project: "proptest",
    source_path: "proptest/src/bits.rs",
    license: "MIT OR Apache-2.0",
    level: 2,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 31.0,
    code: include_str!("../../../challenges/proptest/02-bit-set-trait.rs"),
  },
  Challenge {
    id: "challenge-pt-03",
    title: "Integer bit sets",
    project: "proptest",
    source_path: "proptest/src/bits.rs",
    license: "MIT OR Apache-2.0",
    level: 3,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 33.0,
    code: include_str!("../../../challenges/proptest/03-integer-bit-sets.rs"),
  },
  Challenge {
    id: "challenge-pt-04",
    title: "None strategy",
    project: "proptest",
    source_path: "proptest/src/option.rs",
    license: "MIT OR Apache-2.0",
    level: 4,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 36.0,
    code: include_str!("../../../challenges/proptest/04-none-strategy.rs"),
  },
  Challenge {
    id: "challenge-pt-05",
    title: "Probability",
    project: "proptest",
    source_path: "proptest/src/option.rs",
    license: "MIT OR Apache-2.0",
    level: 5,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 38.0,
    code: include_str!("../../../challenges/proptest/05-probability.rs"),
  },
  Challenge {
    id: "challenge-pt-06",
    title: "Weighted union",
    project: "proptest",
    source_path: "proptest/src/strategy/unions.rs",
    license: "MIT OR Apache-2.0",
    level: 6,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 41.0,
    code: include_str!("../../../challenges/proptest/06-weighted-union.rs"),
  },
  Challenge {
    id: "challenge-tk-01",
    title: "Sync wrapper",
    project: "tokio",
    source_path: "tokio/src/util/sync_wrapper.rs",
    license: "MIT",
    level: 1,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 29.0,
    code: include_str!("../../../challenges/tokio/01-sync-wrapper.rs"),
  },
  Challenge {
    id: "challenge-tk-02",
    title: "Atomic cell",
    project: "tokio",
    source_path: "tokio/src/util/atomic_cell.rs",
    license: "MIT",
    level: 2,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 32.0,
    code: include_str!("../../../challenges/tokio/02-atomic-cell.rs"),
  },
  Challenge {
    id: "challenge-tk-03",
    title: "Rc cell",
    project: "tokio",
    source_path: "tokio/src/util/rc_cell.rs",
    license: "MIT",
    level: 3,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 34.0,
    code: include_str!("../../../challenges/tokio/03-rc-cell.rs"),
  },
  Challenge {
    id: "challenge-tk-04",
    title: "Write all",
    project: "tokio",
    source_path: "tokio/src/io/util/write_all.rs",
    license: "MIT",
    level: 4,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 37.0,
    code: include_str!("../../../challenges/tokio/04-write-all.rs"),
  },
  Challenge {
    id: "challenge-tk-05",
    title: "Pack bits",
    project: "tokio",
    source_path: "tokio/src/util/bit.rs",
    license: "MIT",
    level: 5,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 39.0,
    code: include_str!("../../../challenges/tokio/05-pack-bits.rs"),
  },
  Challenge {
    id: "challenge-tk-06",
    title: "Read exact",
    project: "tokio",
    source_path: "tokio/src/io/util/read_exact.rs",
    license: "MIT",
    level: 6,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 42.0,
    code: include_str!("../../../challenges/tokio/06-read-exact.rs"),
  },
  Challenge {
    id: "challenge-ax-01",
    title: "Not found service",
    project: "axum",
    source_path: "axum/src/routing/not_found.rs",
    license: "MIT",
    level: 1,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 30.0,
    code: include_str!("../../../challenges/axum/01-not-found-service.rs"),
  },
  Challenge {
    id: "challenge-ax-02",
    title: "Layered future",
    project: "axum",
    source_path: "axum/src/handler/future.rs",
    license: "MIT",
    level: 2,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 33.0,
    code: include_str!("../../../challenges/axum/02-layered-future.rs"),
  },
  Challenge {
    id: "challenge-ax-03",
    title: "Append headers",
    project: "axum",
    source_path: "axum-core/src/response/append_headers.rs",
    license: "MIT",
    level: 3,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 35.0,
    code: include_str!("../../../challenges/axum/03-append-headers.rs"),
  },
  Challenge {
    id: "challenge-ax-04",
    title: "Raw query",
    project: "axum",
    source_path: "axum/src/extract/raw_query.rs",
    license: "MIT",
    level: 4,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 38.0,
    code: include_str!("../../../challenges/axum/04-raw-query.rs"),
  },
  Challenge {
    id: "challenge-ax-05",
    title: "Method filter",
    project: "axum",
    source_path: "axum/src/routing/method_filter.rs",
    license: "MIT",
    level: 5,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 40.0,
    code: include_str!("../../../challenges/axum/05-method-filter.rs"),
  },
  Challenge {
    id: "challenge-ax-06",
    title: "From request",
    project: "axum",
    source_path: "axum-core/src/extract/mod.rs",
    license: "MIT",
    level: 6,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 43.0,
    code: include_str!("../../../challenges/axum/06-from-request.rs"),
  },
  Challenge {
    id: "challenge-lp-01",
    title: "Or poisoned",
    project: "leptos",
    source_path: "or_poisoned/src/lib.rs",
    license: "MIT",
    level: 1,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 31.0,
    code: include_str!("../../../challenges/leptos/01-or-poisoned.rs"),
  },
  Challenge {
    id: "challenge-lp-02",
    title: "Effect function",
    project: "leptos",
    source_path: "reactive_graph/src/effect/effect_function.rs",
    license: "MIT",
    level: 2,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 34.0,
    code: include_str!("../../../challenges/leptos/02-effect-function.rs"),
  },
  Challenge {
    id: "challenge-lp-03",
    title: "Next tuple",
    project: "leptos",
    source_path: "next_tuple/src/lib.rs",
    license: "MIT",
    level: 3,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 36.0,
    code: include_str!("../../../challenges/leptos/03-next-tuple.rs"),
  },
  Challenge {
    id: "challenge-lp-04",
    title: "Async transition",
    project: "leptos",
    source_path: "reactive_graph/src/transition.rs",
    license: "MIT",
    level: 4,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 39.0,
    code: include_str!("../../../challenges/leptos/04-async-transition.rs"),
  },
  Challenge {
    id: "challenge-lp-05",
    title: "Show",
    project: "leptos",
    source_path: "leptos/src/show.rs",
    license: "MIT",
    level: 5,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 41.0,
    code: include_str!("../../../challenges/leptos/05-show.rs"),
  },
  Challenge {
    id: "challenge-lp-06",
    title: "Add any attr",
    project: "leptos",
    source_path: "tachys/src/view/add_attr.rs",
    license: "MIT",
    level: 6,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 44.0,
    code: include_str!("../../../challenges/leptos/06-add-any-attr.rs"),
  },
  Challenge {
    id: "challenge-sq-01",
    title: "Postgres database",
    project: "sqlx",
    source_path: "sqlx-postgres/src/database.rs",
    license: "MIT OR Apache-2.0",
    level: 1,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 32.0,
    code: include_str!("../../../challenges/sqlx/01-postgres-database.rs"),
  },
  Challenge {
    id: "challenge-sq-02",
    title: "Protocol query",
    project: "sqlx",
    source_path: "sqlx-postgres/src/message/query.rs",
    license: "MIT OR Apache-2.0",
    level: 2,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 35.0,
    code: include_str!("../../../challenges/sqlx/02-protocol-query.rs"),
  },
  Challenge {
    id: "challenge-sq-03",
    title: "Read buffer",
    project: "sqlx",
    source_path: "sqlx-core/src/io/read_buf.rs",
    license: "MIT OR Apache-2.0",
    level: 3,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 37.0,
    code: include_str!("../../../challenges/sqlx/03-read-buffer.rs"),
  },
  Challenge {
    id: "challenge-sq-04",
    title: "Protocol decode",
    project: "sqlx",
    source_path: "sqlx-core/src/io/decode.rs",
    license: "MIT OR Apache-2.0",
    level: 4,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 40.0,
    code: include_str!("../../../challenges/sqlx/04-protocol-decode.rs"),
  },
  Challenge {
    id: "challenge-sq-05",
    title: "Type info",
    project: "sqlx",
    source_path: "sqlx-core/src/type_info.rs",
    license: "MIT OR Apache-2.0",
    level: 5,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 42.0,
    code: include_str!("../../../challenges/sqlx/05-type-info.rs"),
  },
  Challenge {
    id: "challenge-sq-06",
    title: "Derive renames",
    project: "sqlx",
    source_path: "sqlx-macros-core/src/derives/mod.rs",
    license: "MIT OR Apache-2.0",
    level: 6,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 45.0,
    code: include_str!("../../../challenges/sqlx/06-derive-renames.rs"),
  },
  Challenge {
    id: "challenge-rd-01",
    title: "Standard bool",
    project: "rand",
    source_path: "src/distr/other.rs",
    license: "MIT OR Apache-2.0",
    level: 1,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 33.0,
    code: include_str!("../../../challenges/rand/01-standard-bool.rs"),
  },
  Challenge {
    id: "challenge-rd-02",
    title: "Alphabetic distribution",
    project: "rand",
    source_path: "src/distr/other.rs",
    license: "MIT OR Apache-2.0",
    level: 2,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 36.0,
    code: include_str!("../../../challenges/rand/02-alphabetic.rs"),
  },
  Challenge {
    id: "challenge-rd-03",
    title: "Small RNG adapters",
    project: "rand",
    source_path: "src/rngs/small.rs",
    license: "MIT OR Apache-2.0",
    level: 3,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 38.0,
    code: include_str!("../../../challenges/rand/03-small-rng.rs"),
  },
  Challenge {
    id: "challenge-rd-04",
    title: "Increasing uniform bound",
    project: "rand",
    source_path: "src/seq/increasing_uniform.rs",
    license: "MIT OR Apache-2.0",
    level: 4,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 41.0,
    code: include_str!("../../../challenges/rand/04-increasing-bound.rs"),
  },
  Challenge {
    id: "challenge-rd-05",
    title: "Xoshiro seeding",
    project: "rand",
    source_path: "src/rngs/xoshiro128plusplus.rs",
    license: "MIT OR Apache-2.0",
    level: 5,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 43.0,
    code: include_str!("../../../challenges/rand/05-xoshiro-seed.rs"),
  },
  Challenge {
    id: "challenge-rd-06",
    title: "Weighted index",
    project: "rand",
    source_path: "src/distr/weighted/weighted_index.rs",
    license: "MIT OR Apache-2.0",
    level: 6,
    accuracy_goal: TWO_STAR_ACCURACY,
    wpm_goal: 46.0,
    code: include_str!("../../../challenges/rand/06-weighted-index.rs"),
  },
];

/// All levels in level order.
pub fn all() -> &'static [Challenge] {
  CHALLENGES
}

/// Look up a level by snippet id.
pub fn by_id(id: &str) -> Option<&'static Challenge> {
  CHALLENGES.iter().find(|challenge| challenge.id == id)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn levels_are_ordered_within_projects() {
    let levels = all();
    assert!(!levels.is_empty());
    let mut ids: Vec<&str> = levels.iter().map(|level| level.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), levels.len(), "ids must be unique");

    let mut groups: Vec<(&str, Vec<u32>)> = Vec::new();
    for level in levels {
      match groups.last_mut() {
        Some((project, list)) if *project == level.project => list.push(level.level),
        _ => groups.push((level.project, vec![level.level])),
      }
    }
    for (project, list) in groups {
      assert_eq!(list[0], 1, "{project} must start at level 01");
      assert!(
        list.windows(2).all(|pair| pair[0] < pair[1]),
        "{project} levels must be ordered"
      );
    }
  }

  #[test]
  fn each_project_is_one_contiguous_run() {
    let mut seen: Vec<&str> = Vec::new();
    for level in all() {
      if seen.last() != Some(&level.project) {
        assert!(
          !seen.contains(&level.project),
          "{} is split into multiple runs",
          level.project
        );
        seen.push(level.project);
      }
    }
  }

  #[test]
  fn levels_carry_attribution_and_goals() {
    for level in all() {
      assert!(level.id.starts_with("challenge-"));
      assert!(!level.title.is_empty());
      assert!(!level.project.is_empty());
      assert!(level.source_path.ends_with(".rs"));
      assert!(!level.license.is_empty());
      assert!(level.accuracy_goal > 0.0 && level.accuracy_goal <= 1.0);
      assert!(level.wpm_goal > 0.0);
    }
  }

  #[test]
  fn level_code_is_a_clean_excerpt() {
    for level in all() {
      let lines = level.code.lines().count();
      assert!(
        (10..=40).contains(&lines),
        "{} has {lines} lines, expected 10..=40",
        level.id
      );
      assert!(!level.code.contains('\r'), "{} contains CR", level.id);
      assert!(!level.code.trim().is_empty());
    }
  }

  #[test]
  fn snippet_maps_the_level() {
    let level = &all()[0];
    let snippet = level.snippet();
    assert_eq!(snippet.id, level.id);
    assert_eq!(snippet.source, SnippetSource::Challenge);
    assert_eq!(snippet.title, level.title);
    assert_eq!(snippet.code, level.code.trim_end());
  }

  #[test]
  fn stars_reward_finish_accuracy_and_speed() {
    let level = &all()[0];
    assert_eq!(level.stars(level.wpm_goal, 0.5), 1);
    assert_eq!(level.stars(0.0, level.accuracy_goal), 2);
    assert_eq!(level.stars(level.wpm_goal, level.accuracy_goal), 3);
  }

  #[test]
  fn by_id_finds_levels() {
    let first = all()[0];
    assert_eq!(by_id(first.id).map(|level| level.title), Some(first.title));
    assert!(by_id("challenge-nope").is_none());
  }
}
