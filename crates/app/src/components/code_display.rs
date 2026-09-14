use leptos::prelude::*;

use super::highlight::{HighlightData, ensure_css_injected, get_highlight};

/// DOM id of the current-character span, used for auto-scroll.
pub const CURRENT_CHAR_ID: &str = "rusttype-current-char";

/// Renders the syntect-highlighted target with cursor and wrong-char overlay.
///
/// Highlighting is cached per snippet (`highlight::get_highlight`), so
/// keystrokes never re-tokenize the whole snippet. The untyped tail is one
/// static span per run of same-class characters; only the typed prefix +
/// current character re-render per keystroke (the affected region).
#[component]
pub fn CodeDisplay(code: String, cursor: usize, errors: Vec<usize>) -> impl IntoView {
  let highlight = get_highlight(&code);
  ensure_css_injected(&highlight.css);

  let chars: Vec<char> = code.chars().collect();
  let typed_count = cursor.min(chars.len());
  let tail_start = typed_count + 1;

  let tail = tail_runs(&chars, tail_start, &highlight);

  view! {
      <div class="code-display">
          <pre class="code-block">
              <code>
                  {chars.iter().enumerate().take(tail_start).map(|(i, &ch)| {
                      let class = char_class(i, &chars, cursor, &errors, &highlight);
                      let span_id = (i == cursor).then_some(CURRENT_CHAR_ID);
                      let display = match ch {
                          '\n' => "↵\n".to_string(),
                          '\t' => "    ".to_string(),
                          other => other.to_string(),
                      };
                      view! { <span id=span_id class=class>{display}</span> }
                  }).collect_view()}
                  {tail.into_iter().map(|(tok, text)| {
                      let class = format!("tok-{tok}");
                      view! { <span class=class>{text}</span> }
                  }).collect_view()}
              </code>
          </pre>
      </div>
  }
}

/// Group the untyped tail into runs of consecutive characters that share a
/// syntax class, so the tail stays syntax-colored without one DOM node per
/// character. Tabs and newlines expand exactly as in the typed prefix.
fn tail_runs(chars: &[char], tail_start: usize, highlight: &HighlightData) -> Vec<(usize, String)> {
  let mut runs: Vec<(usize, String)> = Vec::new();
  for (index, &ch) in chars.iter().enumerate().skip(tail_start) {
    let tok = highlight.char_classes.get(index).copied().unwrap_or(0);
    let display = match ch {
      '\n' => "\n".to_string(),
      '\t' => "    ".to_string(),
      other => other.to_string(),
    };
    match runs.last_mut() {
      Some((last_tok, text)) if *last_tok == tok => text.push_str(&display),
      _ => runs.push((tok, display)),
    }
  }
  runs
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

#[cfg(test)]
mod tests {
  use super::*;

  fn highlight_with(classes: Vec<usize>) -> HighlightData {
    HighlightData {
      char_classes: classes,
      css: String::new(),
    }
  }

  #[test]
  fn tail_runs_groups_consecutive_classes() {
    let chars: Vec<char> = "fn main() {}".chars().collect();
    let data = highlight_with(vec![0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 3]);
    let runs = tail_runs(&chars, 4, &data);
    assert_eq!(
      runs,
      vec![
        (1, "ain".to_string()),
        (2, "()".to_string()),
        (3, " {}".to_string()),
      ]
    );
  }

  #[test]
  fn tail_runs_expands_tabs_and_newlines() {
    let chars: Vec<char> = "a\tb\nc".chars().collect();
    let data = highlight_with(vec![0, 0, 0, 0, 0]);
    let runs = tail_runs(&chars, 0, &data);
    assert_eq!(runs, vec![(0, "a    b\nc".to_string())]);
  }

  #[test]
  fn tail_runs_empty_when_cursor_at_end() {
    let chars: Vec<char> = "fn".chars().collect();
    let data = highlight_with(vec![0, 0]);
    assert!(tail_runs(&chars, 2, &data).is_empty());
  }
}
