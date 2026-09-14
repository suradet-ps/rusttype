//! Weak-spot drill generation.
//!
//! Aggregated `worst_tokens` counts (from session history) are turned into a
//! short synthetic snippet that exercises the tokens in real Rust contexts.
//! Generation is deterministic: the same tokens produce the same drill.

use crate::model::{Snippet, SnippetSource};

/// Tokens exercised by a single drill.
pub const MAX_DRILL_TOKENS: usize = 5;

/// Cap on a drillable token length - `worst_tokens` windows are 2-3 chars.
const MAX_TOKEN_LEN: usize = 4;

/// Stable id for the generated drill snippet.
const DRILL_ID: &str = "generated-drill";

/// Select the tokens a drill should exercise.
///
/// Filters out anything that cannot be woven into Rust source (control
/// characters, quotes, backslashes, over-long windows), sorts by error count
/// (descending, token ascending on ties) and keeps the top
/// [`MAX_DRILL_TOKENS`].
pub fn select_tokens(tokens: &[(String, usize)]) -> Vec<String> {
  let mut selected: Vec<&(String, usize)> = tokens
    .iter()
    .filter(|(token, _)| is_drillable(token))
    .collect();
  selected.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
  selected
    .into_iter()
    .map(|(token, _)| token.clone())
    .take(MAX_DRILL_TOKENS)
    .collect()
}

/// Build a drill snippet for `tokens`, or `None` when nothing is drillable.
pub fn generate_drill(tokens: &[(String, usize)]) -> Option<Snippet> {
  let selected = select_tokens(tokens);
  if selected.is_empty() {
    return None;
  }

  let mut body: Vec<String> = Vec::new();
  for token in &selected {
    let line = drill_line(token);
    if !body.contains(&line) {
      body.push(line);
    }
  }

  let indented = body
    .iter()
    .map(|line| format!("    {line}"))
    .collect::<Vec<_>>()
    .join("\n");
  let code = format!("fn drill() {{\n{indented}\n}}");

  Some(Snippet {
    id: DRILL_ID.to_string(),
    source: SnippetSource::Generated,
    title: format!("Drill: {}", selected.join(" ")),
    code,
  })
}

/// Whether a token can be embedded in generated Rust source.
fn is_drillable(token: &str) -> bool {
  !token.is_empty()
    && token.chars().count() <= MAX_TOKEN_LEN
    && token.chars().all(|c| c.is_ascii_graphic() || c == ' ')
    && !token.chars().any(|c| matches!(c, '"' | '\\' | '`'))
}

/// A Rust line that exercises `token`.
fn drill_line(token: &str) -> String {
  let known = match token {
    "::" => Some("let path = std::mem::size_of::<u32>();"),
    "->" => Some("fn step(n: u32) -> u32 { n + 1 }"),
    "=>" => Some(r#"let label = match 0 { 0 => "zero", _ => "many" };"#),
    "{}" => Some("if ready { run(); } else { wait(); }"),
    "()" => Some(r#"fn main() { println!("drill"); }"#),
    "[]" => Some("let first = items[0];"),
    "<>" => Some("fn identity<T>(value: T) -> T { value }"),
    "&" => Some("let slice = &items[..];"),
    "&&" => Some("if ready && armed { start(); }"),
    "||" => Some("let either = left || right;"),
    "?" => Some("let value = parse(input)?;"),
    ";" => Some("let count = 0;"),
    "," => Some("let pair = (0, 1);"),
    "=" => Some("let mut total = 0;"),
    "==" => Some("if count == limit { stop(); }"),
    "!=" => Some("if count != limit { continue(); }"),
    "+" => Some("let next = index + 1;"),
    "-" => Some("let previous = index - 1;"),
    "*" => Some("let area = width * height;"),
    "/" => Some("let half = total / 2;"),
    ":" => Some(r#"let label: &str = "drill";"#),
    "fn" => Some("fn run() { start(); }"),
    "let" => Some("let mut counter = 0;"),
    "mut" => Some("let mut items = Vec::new();"),
    "if" => Some("if ready { start(); }"),
    "else" => Some("if ready { start() } else { wait() }"),
    "match" => Some("match value { _ => 0 };"),
    "impl" => Some("impl Counter { fn next(&mut self) -> u32 { self.value + 1 } }"),
    "pub" => Some("pub fn run() { start(); }"),
    "use" => Some("use std::collections::HashMap;"),
    "self" => Some("fn next(&self) -> u32 { self.value }"),
    "return" => Some("return value;"),
    "true" => Some("let ready = true;"),
    "false" => Some("let ready = false;"),
    "None" => Some("let maybe: Option<u32> = None;"),
    "Some" => Some("let maybe: Option<u32> = Some(1);"),
    "Ok" => Some("let outcome: Result<u32, ()> = Ok(1);"),
    "Err" => Some(r#"let outcome: Result<u32, ()> = Err("nope");"#),
    "Vec" | "vec" => Some("let items: Vec<u32> = vec![1, 2, 3];"),
    "String" => Some(r#"let name: String = "drill".to_string();"#),
    "str" => Some(r#"let text: &str = "drill";"#),
    "u32" => Some("let count: u32 = 0;"),
    "usize" => Some("let index: usize = 0;"),
    _ => None,
  };

  match known {
    Some(line) => line.to_string(),
    None if is_identifier(token) && !is_keyword(token) => format!("let {token} = 0;"),
    None => format!(r#"let sample = "{token}";"#),
  }
}

/// Whether `token` is a valid Rust identifier shape.
fn is_identifier(token: &str) -> bool {
  let mut chars = token.chars();
  match chars.next() {
    Some(first) if first.is_ascii_alphabetic() || first == '_' => {}
    _ => return false,
  }
  chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Whether `token` is a Rust keyword (cannot be used as an identifier).
fn is_keyword(token: &str) -> bool {
  matches!(
    token,
    "as"
      | "async"
      | "await"
      | "box"
      | "break"
      | "const"
      | "continue"
      | "crate"
      | "dyn"
      | "else"
      | "enum"
      | "extern"
      | "false"
      | "fn"
      | "for"
      | "if"
      | "impl"
      | "in"
      | "let"
      | "loop"
      | "match"
      | "mod"
      | "move"
      | "mut"
      | "pub"
      | "ref"
      | "return"
      | "self"
      | "static"
      | "struct"
      | "super"
      | "trait"
      | "true"
      | "type"
      | "unsafe"
      | "use"
      | "where"
      | "while"
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  fn tokens(entries: &[(&str, usize)]) -> Vec<(String, usize)> {
    entries
      .iter()
      .map(|(token, count)| ((*token).to_string(), *count))
      .collect()
  }

  #[test]
  fn select_tokens_filters_and_sorts() {
    let selected = select_tokens(&tokens(&[
      ("::", 3),
      ("et", 5),
      ("\n", 9),
      ("\"", 8),
      ("toolongtoken", 7),
    ]));
    assert_eq!(selected, vec!["et", "::"]);
  }

  #[test]
  fn select_tokens_breaks_ties_alphabetically() {
    let selected = select_tokens(&tokens(&[("bb", 1), ("aa", 1)]));
    assert_eq!(selected, vec!["aa", "bb"]);
  }

  #[test]
  fn select_tokens_caps_kept_tokens() {
    let selected = select_tokens(&tokens(&[
      ("a", 6),
      ("b", 5),
      ("c", 4),
      ("d", 3),
      ("e", 2),
      ("f", 1),
    ]));
    assert_eq!(selected.len(), MAX_DRILL_TOKENS);
  }

  #[test]
  fn generate_drill_rejects_empty_input() {
    assert!(generate_drill(&[]).is_none());
    assert!(generate_drill(&tokens(&[("\n", 4)])).is_none());
  }

  #[test]
  fn generate_drill_uses_known_templates() {
    let drill = generate_drill(&tokens(&[("::", 2)])).expect("drillable token");
    assert_eq!(drill.source, SnippetSource::Generated);
    assert_eq!(drill.id, DRILL_ID);
    assert_eq!(drill.title, "Drill: ::");
    assert!(drill.code.starts_with("fn drill() {\n    "));
    assert!(drill.code.ends_with("\n}"));
    assert!(drill.code.contains("std::mem::size_of"));
  }

  #[test]
  fn generate_drill_falls_back_to_identifiers_and_literals() {
    let drill = generate_drill(&tokens(&[("et", 2), ("as", 1)])).expect("drillable tokens");
    assert!(drill.code.contains("let et = 0;"));
    assert!(drill.code.contains(r#"let sample = "as";"#));
  }

  #[test]
  fn generate_drill_contains_every_selected_token() {
    let input = tokens(&[("::", 3), ("et", 2), ("?", 1)]);
    let selected = select_tokens(&input);
    let drill = generate_drill(&input).expect("drillable tokens");
    for token in selected {
      assert!(drill.code.contains(&token), "missing {token:?} in drill");
    }
  }

  #[test]
  fn generate_drill_is_deterministic() {
    let input = tokens(&[("->", 2), ("{}", 1)]);
    assert_eq!(generate_drill(&input), generate_drill(&input));
  }

  #[test]
  fn generate_drill_deduplicates_lines() {
    let drill = generate_drill(&tokens(&[("Vec", 2), ("vec", 1)])).expect("drillable tokens");
    let lines: Vec<&str> = drill.code.lines().collect();
    let mut unique = lines.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(lines.len(), unique.len());
  }
}
