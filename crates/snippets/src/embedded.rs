use crate::model::{Snippet, SnippetSource};

/// Embedded Rust snippet data loaded at compile time.
///
/// RustType is Rust-only by design: every embedded snippet is Rust source
/// bundled via `include_str!` and enumerated here at compile time.
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
        source: SnippetSource::Embedded,
        title,
        code: code.trim_end().to_string(),
      });
    }

    Self { snippets }
  }

  /// List all embedded snippets.
  pub fn list(&self) -> Vec<Snippet> {
    self.snippets.clone()
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

  #[test]
  fn embedded_snippets_not_empty() {
    let embedded = EmbeddedSnippets::new();
    assert!(!embedded.list().is_empty());
  }

  #[test]
  fn embedded_snippets_are_user_visible_rust() {
    let embedded = EmbeddedSnippets::new();
    for snippet in embedded.list() {
      assert_eq!(snippet.source, SnippetSource::Embedded);
      assert!(!snippet.title.is_empty());
    }
  }

  #[test]
  fn embedded_snippets_have_no_carriage_returns() {
    // CRLF in embedded snippets would softlock strict-mode typing: the
    // untypeable '\r' would precede every newline.
    let embedded = EmbeddedSnippets::new();
    for snippet in embedded.list() {
      assert!(
        !snippet.code.contains('\r'),
        "snippet {} contains CR",
        snippet.id
      );
    }
  }
}
