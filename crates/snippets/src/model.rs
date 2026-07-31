use serde::{Deserialize, Serialize};

/// Supported programming languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
  Rust,
  Python,
  JavaScript,
  TypeScript,
  Go,
  C,
  Cpp,
}

impl Language {
  /// Display name for the language.
  pub fn display_name(&self) -> &'static str {
    match self {
      Self::Rust => "Rust",
      Self::Python => "Python",
      Self::JavaScript => "JavaScript",
      Self::TypeScript => "TypeScript",
      Self::Go => "Go",
      Self::C => "C",
      Self::Cpp => "C++",
    }
  }
}

/// Whether a snippet is embedded in the binary or provided by the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnippetSource {
  Embedded,
  UserProvided,
}

/// A code snippet for typing practice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
  /// Unique identifier.
  pub id: String,
  /// Programming language.
  pub language: Language,
  /// Whether the snippet is embedded or user-provided.
  pub source: SnippetSource,
  /// Human-readable title.
  pub title: String,
  /// Raw code content (indentation is significant).
  pub code: String,
}

/// Errors that can occur when working with snippets.
#[derive(Debug, thiserror::Error)]
pub enum SnippetError {
  #[error("snippet code must not be empty")]
  Empty,
  #[error("snippet code exceeds maximum length of {0} characters")]
  TooLong(usize),
  #[error("localStorage is unavailable")]
  StorageUnavailable,
  #[error("failed to serialize/deserialize snippet data")]
  SerializationFailed,
}
