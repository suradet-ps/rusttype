use leptos::either::Either;
use leptos::prelude::*;

#[component]
pub fn ResultsSummary(
  stats: engine::SessionStats,
  on_restart: impl Fn() + Clone + 'static,
) -> impl IntoView {
  let worst_tokens = stats.worst_tokens.clone();
  let has_tokens = !worst_tokens.is_empty();

  let wpm = format!("{:.1}", stats.wpm);
  let acc = format!("{:.1}%", stats.accuracy * 100.0);
  let errors = stats.error_count;
  let duration = format!("{:.1}s", stats.duration_ms / 1000.0);

  view! {
      <div class="results-summary">
          <h2 class="results-title">"Session Complete!"</h2>

          <div class="results-grid">
              <div class="result-card">
                  <span class="result-label">"WPM"</span>
                  <span class="result-value">{wpm}</span>
              </div>
              <div class="result-card">
                  <span class="result-label">"Accuracy"</span>
                  <span class="result-value">{acc}</span>
              </div>
              <div class="result-card">
                  <span class="result-label">"Errors"</span>
                  <span class="result-value">{errors}</span>
              </div>
              <div class="result-card">
                  <span class="result-label">"Duration"</span>
                  <span class="result-value">{duration}</span>
              </div>
          </div>

          {move || {
              if has_tokens {
                  let tokens = worst_tokens.clone();
                  Either::Left(view! {
                      <div class="worst-tokens">
                          <h3>"Most mistyped tokens"</h3>
                          <div class="token-list">
                              {tokens.into_iter().map(|(token, count)| {
                                  view! {
                                      <span class="token-item">
                                          <code>{token}</code>
                                          <span class="token-count">{count}"x"</span>
                                      </span>
                                  }
                              }).collect_view()}
                          </div>
                      </div>
                  })
              } else {
                  Either::Right(view! { <p class="no-errors">"No errors — perfect run!"</p> })
              }
          }}

          <button
              class="btn btn-primary"
              on:click=move |_| (on_restart)()
          >
              "Try Again"
          </button>
      </div>
  }
}
