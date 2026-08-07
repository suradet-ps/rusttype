use leptos::prelude::*;

/// Live statistics bar shown while typing.
#[component]
pub fn StatsBar(wpm: f64, accuracy: f64, error_count: usize, progress: f64) -> impl IntoView {
  view! {
      <div class="stats-bar">
          <div class="stat">
              <span class="stat-label">"WPM"</span>
              <span class="stat-value">{wpm as i32}</span>
          </div>
          <div class="stat">
              <span class="stat-label">"Accuracy"</span>
              <span class="stat-value">{format!("{:.1}%", accuracy)}</span>
          </div>
          <div class="stat">
              <span class="stat-label">"Errors"</span>
              <span class="stat-value">{error_count}</span>
          </div>
          <div class="progress-bar">
              <div class="progress-fill" style:width={format!("{:.0}%", progress * 100.0)}></div>
          </div>
      </div>
  }
}
