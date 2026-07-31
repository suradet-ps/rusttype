use crate::model::{Snippet, SnippetSource};

/// Embedded snippet data loaded at compile time.
pub struct EmbeddedSnippets {
  snippets: Vec<Snippet>,
}

impl EmbeddedSnippets {
  /// Create the embedded snippet collection from compile-time sources.
  pub fn new() -> Self {
    let mut snippets = Vec::new();
    let mut id = 0;

    let rust_files: &[(&str, &str)] = &[
      (
        "hello_world",
        include_str!("../../../snippets-data/rust/hello_world.rs"),
      ),
      (
        "struct_impl",
        include_str!("../../../snippets-data/rust/struct_impl.rs"),
      ),
      (
        "error_handling",
        include_str!("../../../snippets-data/rust/error_handling.rs"),
      ),
      (
        "iterators",
        include_str!("../../../snippets-data/rust/iterators.rs"),
      ),
      (
        "traits",
        include_str!("../../../snippets-data/rust/traits.rs"),
      ),
      (
        "pattern_matching",
        include_str!("../../../snippets-data/rust/pattern_matching.rs"),
      ),
      (
        "lifetimes",
        include_str!("../../../snippets-data/rust/lifetimes.rs"),
      ),
      (
        "async_await",
        include_str!("../../../snippets-data/rust/async_await.rs"),
      ),
    ];

    for (name, code) in rust_files {
      id += 1;
      let title = name
        .replace('_', " ")
        .chars()
        .enumerate()
        .map(|(i, c)| {
          if i == 0 {
            c.to_uppercase().to_string()
          } else {
            c.to_string()
          }
        })
        .collect::<String>();

      snippets.push(Snippet {
        id: format!("embedded-rust-{id:02}"),
        language: crate::model::Language::Rust,
        source: SnippetSource::Embedded,
        title,
        code: code.trim_end().to_string(),
      });
    }

    Self { snippets }
  }

  /// List all embedded snippets, optionally filtered by language.
  pub fn list(&self, language: Option<crate::model::Language>) -> Vec<Snippet> {
    self
      .snippets
      .iter()
      .filter(|s| language.is_none_or(|l| s.language == l))
      .cloned()
      .collect()
  }
}

impl Default for EmbeddedSnippets {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::Language;

  #[test]
  fn embedded_snippets_not_empty() {
    let embedded = EmbeddedSnippets::new();
    assert!(!embedded.list(None).is_empty());
  }

  #[test]
  fn embedded_snippets_all_rust() {
    let embedded = EmbeddedSnippets::new();
    for snippet in embedded.list(None) {
      assert_eq!(snippet.language, Language::Rust);
      assert_eq!(snippet.source, SnippetSource::Embedded);
    }
  }

  #[test]
  fn filter_by_language() {
    let embedded = EmbeddedSnippets::new();
    let rust_only = embedded.list(Some(Language::Rust));
    let python_only = embedded.list(Some(Language::Python));
    assert!(!rust_only.is_empty());
    assert!(python_only.is_empty());
  }
}
