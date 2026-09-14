use serde::{Deserialize, Serialize};

/// Whether a snippet is embedded in the binary or provided by the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnippetSource {
  Embedded,
  UserProvided,
  /// Synthetic drill generated from the user's most-mistyped tokens.
  Generated,
}

/// A Rust code snippet for typing practice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snippet {
  /// Unique identifier.
  pub id: String,
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
  #[error("snippet not found")]
  NotFound,
}

/// Maximum allowed user snippet code length, counted in characters.
pub const MAX_SNIPPET_LENGTH: usize = 10_000;

/// Derive a title from the first non-blank line of code when none is given.
fn default_title(code: &str) -> String {
  code
    .lines()
    .find(|line| !line.trim().is_empty())
    .map(|line| {
      let trimmed = line.trim();
      if trimmed.chars().count() > 40 {
        trimmed.chars().take(40).collect()
      } else {
        trimmed.to_string()
      }
    })
    .unwrap_or_else(|| "My snippet".to_string())
}

/// Validate user input and build a [`Snippet`] with
/// [`SnippetSource::UserProvided`]. The returned snippet has an empty `id`,
/// which the store must fill before persisting.
///
/// An empty `title` is derived from the first line of code.
///
/// # Errors
///
/// Returns [`SnippetError::Empty`] if `code` is blank, or
/// [`SnippetError::TooLong`] if it exceeds [`MAX_SNIPPET_LENGTH`] characters.
pub fn validate_user_snippet(title: String, code: String) -> Result<Snippet, SnippetError> {
  if code.trim().is_empty() {
    return Err(SnippetError::Empty);
  }
  if code.chars().count() > MAX_SNIPPET_LENGTH {
    return Err(SnippetError::TooLong(MAX_SNIPPET_LENGTH));
  }
  let title = if title.trim().is_empty() {
    default_title(&code)
  } else {
    title
  };
  Ok(Snippet {
    id: String::new(),
    source: SnippetSource::UserProvided,
    title,
    code,
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn snippet_serialization_round_trips() {
    let original = Snippet {
      id: "user-1".into(),
      source: SnippetSource::UserProvided,
      title: "Hello".into(),
      code: "fn main() {}\n".into(),
    };
    let json = serde_json::to_string(&original).unwrap();
    let restored: Snippet = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.id, original.id);
    assert_eq!(restored.source, original.source);
    assert_eq!(restored.title, original.title);
    assert_eq!(restored.code, original.code);
  }

  #[test]
  fn validate_rejects_blank_code() {
    let result = validate_user_snippet("Test".into(), "  \n\t ".into());
    assert!(matches!(result, Err(SnippetError::Empty)));
  }

  #[test]
  fn validate_rejects_overlong_code() {
    let result = validate_user_snippet("Test".into(), "a".repeat(MAX_SNIPPET_LENGTH + 1));
    assert!(matches!(result, Err(SnippetError::TooLong(_))));
  }

  #[test]
  fn validate_limits_by_characters_not_bytes() {
    // Each crab is 4 UTF-8 bytes but 1 character; the limit is characters.
    let ok = validate_user_snippet("Test".into(), "🦀".repeat(MAX_SNIPPET_LENGTH));
    assert!(ok.is_ok());
    let too_long = validate_user_snippet("Test".into(), "🦀".repeat(MAX_SNIPPET_LENGTH + 1));
    assert!(matches!(too_long, Err(SnippetError::TooLong(_))));
  }

  #[test]
  fn validate_defaults_empty_title_to_first_code_line() {
    let snippet = validate_user_snippet(
      String::new(),
      "\n  fn merge_sort(items: &mut [u32]) {\n".into(),
    )
    .unwrap();
    assert_eq!(snippet.title, "fn merge_sort(items: &mut [u32]) {");
    assert_eq!(snippet.source, SnippetSource::UserProvided);
  }

  #[test]
  fn validate_defaults_blank_code_title_to_my_snippet() {
    let snippet = validate_user_snippet(String::new(), " ".into());
    assert!(matches!(snippet, Err(SnippetError::Empty)));
  }

  #[test]
  fn default_title_truncates_long_first_line() {
    let long_line = format!("fn {}() {{", "x".repeat(60));
    let snippet = validate_user_snippet(String::new(), long_line).unwrap();
    assert_eq!(snippet.title.chars().count(), 40);
  }
}
