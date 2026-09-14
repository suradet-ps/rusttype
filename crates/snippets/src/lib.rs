//! Snippet model, embedded snippet loader, user-provided snippet storage.
//!
//! RustType is Rust-only: snippets carry no language field.

mod challenge;
mod drill;
mod embedded;
mod model;
mod store;

pub use challenge::{
  Challenge, TWO_STAR_ACCURACY, all as all_challenges, by_id as challenge_by_id,
};
pub use drill::{generate_drill, select_tokens};
pub use embedded::EmbeddedSnippets;
pub use model::{MAX_SNIPPET_LENGTH, Snippet, SnippetError, SnippetSource, validate_user_snippet};
pub use store::SnippetStore;
