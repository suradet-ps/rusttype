use leptos::prelude::*;

use super::highlight::{HighlightData, ensure_css_injected, get_highlight};

/// Renders the syntect-highlighted target with cursor and wrong-char overlay.
///
/// Highlighting is cached per snippet (`highlight::get_highlight`), so
/// keystrokes never re-tokenize the whole snippet. The untyped tail is a
/// single static text node; only the typed prefix + current character are
/// re-rendered per keystroke (the affected region).
#[component]
pub fn CodeDisplay(
  code: String,
  language: snippets::Language,
  cursor: usize,
  errors: Vec<usize>,
) -> impl IntoView {
  let highlight = get_highlight(&code, language);
  ensure_css_injected(&highlight.css);

  let chars: Vec<char> = code.chars().collect();
  let typed_count = cursor.min(chars.len());
  let tail_start = typed_count + 1;

  let mut tail_text = String::new();
  for ch in chars.iter().skip(tail_start) {
    match ch {
      '\t' => tail_text.push_str("    "),
      '\n' => tail_text.push('\n'),
      other => tail_text.push(*other),
    }
  }

  view! {
      <div class="code-display">
          <pre class="code-block">
              <code>
                  {chars.iter().enumerate().take(tail_start).map(|(i, &ch)| {
                      let class = char_class(i, &chars, cursor, &errors, &highlight);
                      let display = match ch {
                          '\n' => "↵\n".to_string(),
                          '\t' => "    ".to_string(),
                          other => other.to_string(),
                      };
                      view! { <span class=class>{display}</span> }
                  }).collect_view()}
                  <span class="char-tail">{tail_text}</span>
              </code>
          </pre>
      </div>
  }
}

/// CSS class for a single character combining syntax, state, and wrong-type.
fn char_class(
  index: usize,
  chars: &[char],
  cursor: usize,
  errors: &[usize],
  highlight: &HighlightData,
) -> String {
  let tok = format!(
    "tok-{}",
    highlight.char_classes.get(index).copied().unwrap_or(0)
  );
  let state = if errors.contains(&index) {
    if matches!(chars[index], ' ' | '\t' | '\n') {
      "char char-wrong char-wrong-ws"
    } else {
      "char char-wrong"
    }
  } else if index == cursor {
    "char char-current"
  } else if index < cursor {
    "char char-typed"
  } else {
    "char"
  };
  format!("{tok} {state}")
}
