use leptos::prelude::*;

#[component]
pub fn CodeDisplay(target: Vec<char>, cursor: usize, errors: Vec<usize>) -> impl IntoView {
  let target_str: String = target.iter().collect();

  view! {
      <div class="code-display">
          <pre class="code-block">
              <code>
                  {target_str.chars().enumerate().map(|(i, ch)| {
                      let is_current = i == cursor;
                      let is_wrong = errors.contains(&i);
                      let is_typed = i < cursor;

                      let class = if is_wrong {
                          "char char-wrong"
                      } else if is_current {
                          "char char-current"
                      } else if is_typed {
                          "char char-typed"
                      } else {
                          "char"
                      };

                      let display = if ch == '\n' {
                          "↵\n".to_string()
                      } else if ch == ' ' {
                          " ".to_string()
                      } else if ch == '\t' {
                          "    ".to_string()
                      } else {
                          ch.to_string()
                      };

                      view! {
                          <span class=class>{display}</span>
                      }
                  }).collect_view()}
              </code>
          </pre>
      </div>
  }
}
