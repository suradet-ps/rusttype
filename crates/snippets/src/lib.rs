//! Snippet model, embedded snippet loader, user-provided snippet storage.

mod embedded;
mod model;
mod store;

pub use embedded::EmbeddedSnippets;
pub use model::{Language, Snippet, SnippetError, SnippetSource};
pub use store::SnippetStore;
