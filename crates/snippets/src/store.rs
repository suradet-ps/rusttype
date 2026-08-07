use crate::model::{Snippet, SnippetError, validate_user_snippet};

/// Trait for storing and retrieving snippets.
pub trait SnippetStore {
  /// List all stored snippets.
  fn list(&self) -> Vec<Snippet>;

  /// Add a user-provided snippet after validation.
  ///
  /// # Errors
  ///
  /// Returns [`SnippetError::Empty`] if code is empty, or
  /// [`SnippetError::TooLong`] if code exceeds the maximum length.
  fn add_user_snippet(&mut self, title: String, code: String) -> Result<Snippet, SnippetError>;

  /// Remove a snippet by ID.
  ///
  /// # Errors
  ///
  /// Returns [`SnippetError::NotFound`] if the snippet does not exist.
  fn remove(&mut self, id: &str) -> Result<(), SnippetError>;
}

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
  fn list(&self) -> Vec<Snippet> {
    self.user_snippets.clone()
  }

  fn add_user_snippet(&mut self, title: String, code: String) -> Result<Snippet, SnippetError> {
    let mut snippet = validate_user_snippet(title, code)?;
    snippet.id = format!("user-{:04}", self.next_id);
    self.next_id += 1;
    self.user_snippets.push(snippet.clone());
    Ok(snippet)
  }

  fn remove(&mut self, id: &str) -> Result<(), SnippetError> {
    let len_before = self.user_snippets.len();
    self.user_snippets.retain(|s| s.id != id);
    if self.user_snippets.len() == len_before {
      Err(SnippetError::NotFound)
    } else {
      Ok(())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::MAX_SNIPPET_LENGTH;

  #[test]
  fn add_valid_snippet() {
    let mut store = MemorySnippetStore::new();
    let snippet = store
      .add_user_snippet("Test".into(), "fn main() {}".into())
      .unwrap();
    assert_eq!(snippet.title, "Test");
  }

  #[test]
  fn add_empty_snippet_fails() {
    let mut store = MemorySnippetStore::new();
    let result = store.add_user_snippet("Test".into(), String::new());
    assert!(matches!(result, Err(SnippetError::Empty)));
  }

  #[test]
  fn add_blank_code_fails() {
    let mut store = MemorySnippetStore::new();
    let result = store.add_user_snippet("Test".into(), " \n ".into());
    assert!(matches!(result, Err(SnippetError::Empty)));
  }

  #[test]
  fn add_too_long_snippet_fails() {
    let mut store = MemorySnippetStore::new();
    let code = "a".repeat(MAX_SNIPPET_LENGTH + 1);
    let result = store.add_user_snippet("Test".into(), code);
    assert!(matches!(result, Err(SnippetError::TooLong(_))));
  }

  #[test]
  fn add_limits_by_characters_not_bytes() {
    let mut store = MemorySnippetStore::new();
    let result = store.add_user_snippet("Test".into(), "🦀".repeat(MAX_SNIPPET_LENGTH));
    assert!(result.is_ok());
    let too_long = store.add_user_snippet("Test".into(), "🦀".repeat(MAX_SNIPPET_LENGTH + 1));
    assert!(matches!(too_long, Err(SnippetError::TooLong(_))));
  }

  #[test]
  fn empty_title_derives_from_first_code_line() {
    let mut store = MemorySnippetStore::new();
    let snippet = store
      .add_user_snippet(String::new(), "impl fmt::Display for Point {".into())
      .unwrap();
    assert_eq!(snippet.title, "impl fmt::Display for Point {");
  }

  #[test]
  fn list_returns_all_user_snippets() {
    let mut store = MemorySnippetStore::new();
    store
      .add_user_snippet("One".into(), "fn a() {}".into())
      .unwrap();
    store
      .add_user_snippet("Two".into(), "fn b() {}".into())
      .unwrap();
    assert_eq!(store.list().len(), 2);
  }

  #[test]
  fn remove_snippet() {
    let mut store = MemorySnippetStore::new();
    let snippet = store
      .add_user_snippet("Test".into(), "code".into())
      .unwrap();
    store.remove(&snippet.id).unwrap();
    assert!(store.list().is_empty());
  }

  #[test]
  fn remove_nonexistent_fails() {
    let mut store = MemorySnippetStore::new();
    let result = store.remove("nonexistent");
    assert!(matches!(result, Err(SnippetError::NotFound)));
  }
}
