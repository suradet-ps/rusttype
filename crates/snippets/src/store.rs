use crate::model::{Snippet, SnippetError};

/// Trait for storing and retrieving snippets.
pub trait SnippetStore {
  /// List all snippets, optionally filtered by language.
  fn list(&self, language: Option<crate::model::Language>) -> Vec<Snippet>;

  /// Add a user-provided snippet after validation.
  ///
  /// # Errors
  ///
  /// Returns [`SnippetError::Empty`] if code is empty, or
  /// [`SnippetError::TooLong`] if code exceeds the maximum length.
  fn add_user_snippet(
    &mut self,
    title: String,
    language: crate::model::Language,
    code: String,
  ) -> Result<Snippet, SnippetError>;

  /// Remove a snippet by ID.
  ///
  /// # Errors
  ///
  /// Returns an error if the snippet does not exist.
  fn remove(&mut self, id: &str) -> Result<(), SnippetError>;
}

/// Maximum allowed snippet code length (characters).
#[allow(dead_code)]
const MAX_SNIPPET_LENGTH: usize = 10_000;

/// In-memory snippet store combining embedded and user-provided snippets.
#[allow(dead_code)]
pub struct MemorySnippetStore {
  user_snippets: Vec<Snippet>,
  next_id: usize,
}

impl MemorySnippetStore {
  /// Create a new empty store.
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self {
      user_snippets: Vec::new(),
      next_id: 1,
    }
  }
}

impl Default for MemorySnippetStore {
  fn default() -> Self {
    Self::new()
  }
}

impl SnippetStore for MemorySnippetStore {
  fn list(&self, language: Option<crate::model::Language>) -> Vec<Snippet> {
    self
      .user_snippets
      .iter()
      .filter(|s| language.is_none_or(|l| s.language == l))
      .cloned()
      .collect()
  }

  fn add_user_snippet(
    &mut self,
    title: String,
    language: crate::model::Language,
    code: String,
  ) -> Result<Snippet, SnippetError> {
    if code.is_empty() {
      return Err(SnippetError::Empty);
    }
    if code.len() > MAX_SNIPPET_LENGTH {
      return Err(SnippetError::TooLong(MAX_SNIPPET_LENGTH));
    }

    let snippet = Snippet {
      id: format!("user-{:04}", self.next_id),
      language,
      source: crate::model::SnippetSource::UserProvided,
      title,
      code,
    };
    self.next_id += 1;
    self.user_snippets.push(snippet.clone());
    Ok(snippet)
  }

  fn remove(&mut self, id: &str) -> Result<(), SnippetError> {
    let len_before = self.user_snippets.len();
    self.user_snippets.retain(|s| s.id != id);
    if self.user_snippets.len() == len_before {
      Err(SnippetError::StorageUnavailable)
    } else {
      Ok(())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::Language;

  #[test]
  fn add_valid_snippet() {
    let mut store = MemorySnippetStore::new();
    let snippet = store
      .add_user_snippet("Test".into(), Language::Rust, "fn main() {}".into())
      .unwrap();
    assert_eq!(snippet.title, "Test");
    assert_eq!(snippet.language, Language::Rust);
  }

  #[test]
  fn add_empty_snippet_fails() {
    let mut store = MemorySnippetStore::new();
    let result = store.add_user_snippet("Test".into(), Language::Rust, String::new());
    assert!(matches!(result, Err(SnippetError::Empty)));
  }

  #[test]
  fn add_too_long_snippet_fails() {
    let mut store = MemorySnippetStore::new();
    let code = "a".repeat(MAX_SNIPPET_LENGTH + 1);
    let result = store.add_user_snippet("Test".into(), Language::Rust, code);
    assert!(matches!(result, Err(SnippetError::TooLong(_))));
  }

  #[test]
  fn list_filters_by_language() {
    let mut store = MemorySnippetStore::new();
    store
      .add_user_snippet("Rust".into(), Language::Rust, "fn main() {}".into())
      .unwrap();
    store
      .add_user_snippet("Python".into(), Language::Python, "print('hi')".into())
      .unwrap();

    let rust = store.list(Some(Language::Rust));
    let python = store.list(Some(Language::Python));
    assert_eq!(rust.len(), 1);
    assert_eq!(python.len(), 1);
  }

  #[test]
  fn remove_snippet() {
    let mut store = MemorySnippetStore::new();
    let snippet = store
      .add_user_snippet("Test".into(), Language::Rust, "code".into())
      .unwrap();
    store.remove(&snippet.id).unwrap();
    assert!(store.list(None).is_empty());
  }

  #[test]
  fn remove_nonexistent_fails() {
    let mut store = MemorySnippetStore::new();
    let result = store.remove("nonexistent");
    assert!(result.is_err());
  }
}
