//! Syntax highlighting via syntect, cached per snippet.
//!
//! Tokenization happens once per snippet (cached) instead of once per
//! keystroke. The CSS for the fixed theme is generated at runtime from the
//! embedded `.tmTheme` file (reused from the CodeShot project) and injected
//! into the document head — no hex values are hardcoded in components, so the
//! style.css hex-token contract stays intact.

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex, OnceLock};

use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, FontStyle, Style, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

/// Bundled theme, copied from the CodeShot project (light UI-friendly).
const THEME_DATA: &str = include_str!("../../themes/github-light.tmTheme");

/// Per-snippet cached highlight data.
pub struct HighlightData {
  /// Syntax class index for each character of the target (index-aligned with `code.chars()`).
  pub char_classes: Vec<usize>,
  /// CSS rules for each syntax class index.
  pub css: String,
}

/// Packed RGB + font style, deduplicated into a CSS class.
#[derive(Clone, Copy, PartialEq)]
struct TokenStyle {
  color: Color,
  bold: bool,
  italic: bool,
}

fn syntax_set() -> &'static SyntaxSet {
  static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
  SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn theme() -> &'static syntect::highlighting::Theme {
  static THEME: OnceLock<syntect::highlighting::Theme> = OnceLock::new();
  THEME.get_or_init(|| {
    ThemeSet::load_from_reader(&mut Cursor::new(THEME_DATA.as_bytes()))
      .expect("invariant: bundled .tmTheme file parses")
  })
}

fn style_index(style: Style, palette: &mut Vec<TokenStyle>) -> usize {
  let token = TokenStyle {
    color: style.foreground,
    bold: style.font_style.contains(FontStyle::BOLD),
    italic: style.font_style.contains(FontStyle::ITALIC),
  };
  if let Some((i, _)) = palette.iter().enumerate().find(|(_, t)| **t == token) {
    i
  } else {
    palette.push(token);
    palette.len() - 1
  }
}

fn build_css(palette: &[TokenStyle]) -> String {
  let mut css = String::new();
  for (i, token) in palette.iter().enumerate() {
    let Color { r, g, b, .. } = token.color;
    let mut rule = format!(".tok-{i}{{color:rgb({r},{g},{b});");
    if token.bold {
      rule.push_str("font-weight:600;");
    }
    if token.italic {
      rule.push_str("font-style:italic;");
    }
    rule.push('}');
    css.push_str(&rule);
  }
  css
}

/// RustType is Rust-only, so the grammar is fixed: the `.rs` extension maps
/// to the bundled Rust syntax definition.
fn rust_syntax() -> &'static SyntaxReference {
  syntax_set()
    .find_syntax_by_extension("rs")
    .expect("invariant: Rust grammar is bundled with syntect defaults")
}

/// Tokenize `code` as Rust and return per-char classes + theme CSS.
fn tokenize(code: &str) -> HighlightData {
  let syntax = rust_syntax();
  let theme = theme();
  let mut highlighter = HighlightLines::new(syntax, theme);

  let mut palette: Vec<TokenStyle> = Vec::new();
  let mut char_classes: Vec<usize> = Vec::with_capacity(code.len());

  for line in LinesWithEndings::from(code) {
    let ops = highlighter
      .highlight_line(line, syntax_set())
      .expect("invariant: bundled grammar highlights any input");
    for (style, text) in ops {
      let idx = style_index(style, &mut palette);
      char_classes.extend(std::iter::repeat_n(idx, text.chars().count()));
    }
  }

  HighlightData {
    char_classes,
    css: build_css(&palette),
  }
}

static CACHE: OnceLock<Mutex<HashMap<String, Arc<HighlightData>>>> = OnceLock::new();

/// Returns cached highlight data for the given snippet, tokenizing on first use.
///
/// The full snippet is tokenized exactly once per code string; every
/// subsequent keystroke reuses the cached classes so only the affected region
/// of the view re-renders.
pub fn get_highlight(code: &str) -> Arc<HighlightData> {
  let key = code.to_string();
  let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
  let mut guard = cache
    .lock()
    .expect("invariant: single-threaded WASM, no concurrent panics");
  if let Some(hit) = guard.get(&key) {
    return Arc::clone(hit);
  }
  let data = Arc::new(tokenize(code));
  guard.insert(key, Arc::clone(&data));
  data
}

static INJECTED: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

/// Inject the generated token CSS into the document head once per style sheet.
pub fn ensure_css_injected(css: &str) {
  if css.is_empty() {
    return;
  }
  let injected = INJECTED.get_or_init(|| Mutex::new(Vec::new()));
  let mut guard = injected
    .lock()
    .expect("invariant: single-threaded WASM, no concurrent panics");
  if guard.iter().any(|existing| existing == css) {
    return;
  }
  let style = web_sys::window()
    .expect("invariant: browser environment always provides window")
    .document()
    .expect("invariant: browser environment always provides document")
    .create_element("style")
    .expect("invariant: createElement succeeds in browser");
  style.set_text_content(Some(css));
  let head = web_sys::window()
    .expect("invariant: browser environment always provides window")
    .document()
    .expect("invariant: browser environment always provides document")
    .head()
    .expect("invariant: document always has a head");
  head
    .append_child(&style)
    .expect("invariant: appending a style element to head succeeds");
  guard.push(css.to_string());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tokenize_assigns_classes_for_all_chars() {
    let data = tokenize("fn main() {}\n");
    assert_eq!(data.char_classes.len(), "fn main() {}\n".chars().count());
    assert!(data.char_classes.iter().all(|c| *c < 64));
  }

  #[test]
  fn tokenize_empty_input() {
    let data = tokenize("");
    assert!(data.char_classes.is_empty());
  }

  #[test]
  fn tokenize_multibyte_aligns_by_char() {
    let code = "// ไทย\r\nfn main() {}";
    let data = tokenize(code);
    assert_eq!(data.char_classes.len(), code.chars().count());
  }

  #[test]
  fn cache_returns_same_data() {
    let a = get_highlight("fn main() {}");
    let b = get_highlight("fn main() {}");
    assert!(Arc::ptr_eq(&a, &b));
  }

  #[test]
  fn build_css_has_rule_per_class() {
    let palette = vec![
      TokenStyle {
        color: Color {
          r: 0,
          g: 0,
          b: 0,
          a: 255,
        },
        bold: true,
        italic: false,
      },
      TokenStyle {
        color: Color {
          r: 1,
          g: 2,
          b: 3,
          a: 255,
        },
        bold: false,
        italic: true,
      },
    ];
    let css = build_css(&palette);
    assert!(css.contains(".tok-0{color:rgb(0,0,0);font-weight:600;"));
    assert!(css.contains(".tok-1{color:rgb(1,2,3);font-style:italic;"));
  }
}
