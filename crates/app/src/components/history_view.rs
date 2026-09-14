//! Session history: WPM and accuracy over time.

use leptos::either::Either;
use leptos::prelude::*;
use wasm_bindgen::JsValue;

use crate::history::{History, SessionRecord};

/// Sessions drawn in the chart (newest last).
const CHART_SESSIONS: usize = 30;
/// Sessions listed under the chart (newest first).
const LIST_SESSIONS: usize = 20;

#[component]
pub fn HistoryView(on_back: impl Fn() + Clone + Send + Sync + 'static) -> impl IntoView {
  let history = History::load();
  let total = history.total_sessions();
  let has_sessions = total > 0;
  let best_wpm = format!("{:.1}", history.best_wpm());
  let average_wpm = format!("{:.1}", history.average_wpm());
  let average_accuracy = format!("{:.1}%", history.average_accuracy() * 100.0);
  let session_label = if total == 1 {
    "1 session".to_string()
  } else {
    format!("{total} sessions")
  };

  let chart_max = history.best_wpm();
  let chart: Vec<SessionRecord> = history
    .recent(CHART_SESSIONS)
    .into_iter()
    .rev()
    .cloned()
    .collect();
  let list: Vec<SessionRecord> = history.recent(LIST_SESSIONS).into_iter().cloned().collect();

  view! {
      <div class="history-view">
          <div class="history-header">
              <button class="btn btn-tertiary btn-sm" on:click=move |_| on_back()>
                  "All snippets"
              </button>
              <h2 class="section-title">"Session history"</h2>
              <span class="history-count">{session_label}</span>
          </div>

          {move || {
              if !has_sessions {
                  return Either::Left(view! {
                      <p class="history-empty">
                          "No sessions yet. Finish a snippet to start your history."
                      </p>
                  });
              }
              Either::Right(view! {
                  <div class="history-body">
                      <div class="history-grid">
                          <div class="result-card">
                              <span class="result-label">"Sessions"</span>
                              <span class="result-value">{total}</span>
                          </div>
                          <div class="result-card">
                              <span class="result-label">"Best WPM"</span>
                              <span class="result-value">{best_wpm.clone()}</span>
                          </div>
                          <div class="result-card">
                              <span class="result-label">"Avg WPM"</span>
                              <span class="result-value">{average_wpm.clone()}</span>
                          </div>
                          <div class="result-card">
                              <span class="result-label">"Avg accuracy"</span>
                              <span class="result-value">{average_accuracy.clone()}</span>
                          </div>
                      </div>

                      <div class="history-chart" role="img" aria-label="WPM per session">
                          {chart.iter().map(|record| {
                              let height = bar_percent(record.wpm, chart_max);
                              let tooltip = format!(
                                  "{:.1} WPM, {:.1}% accuracy",
                                  record.wpm,
                                  record.accuracy * 100.0,
                              );
                              view! {
                                  <div
                                      class="history-bar"
                                      style:height=format!("{height:.1}%")
                                      title=tooltip
                                  ></div>
                              }
                          }).collect_view()}
                      </div>

                      <div class="history-list">
                          {list.iter().map(|record| {
                              view! {
                                  <div class="history-row">
                                      <span class="history-row-title">
                                          {record.snippet_title.clone()}
                                      </span>
                                      <span class="history-row-date">
                                          {format_timestamp(record.completed_at_ms)}
                                      </span>
                                      <span class="history-metric">
                                          <span class="history-metric-label">"WPM"</span>
                                          <span class="history-metric-value">
                                              {format!("{:.1}", record.wpm)}
                                          </span>
                                      </span>
                                      <span class="history-metric">
                                          <span class="history-metric-label">"Acc"</span>
                                          <span class="history-metric-value">
                                              {format!("{:.1}%", record.accuracy * 100.0)}
                                          </span>
                                      </span>
                                  </div>
                              }
                          }).collect_view()}
                      </div>
                  </div>
              })
          }}
      </div>
  }
}

/// Bar height as a percentage of the chart maximum.
///
/// Floored so a slow session still shows a visible stub.
fn bar_percent(wpm: f64, max_wpm: f64) -> f64 {
  if max_wpm <= 0.0 {
    return 2.0;
  }
  (wpm / max_wpm * 100.0).max(2.0)
}

/// Format a Unix-epoch millisecond timestamp as local `YYYY-MM-DD HH:MM`.
fn format_timestamp(ms: f64) -> String {
  let date = js_sys::Date::new(&JsValue::from_f64(ms));
  format!(
    "{:04}-{:02}-{:02} {:02}:{:02}",
    date.get_full_year(),
    date.get_month() + 1,
    date.get_date(),
    date.get_hours(),
    date.get_minutes(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bar_percent_scales_to_max() {
    assert_eq!(bar_percent(50.0, 100.0), 50.0);
    assert_eq!(bar_percent(100.0, 100.0), 100.0);
  }

  #[test]
  fn bar_percent_has_visible_floor() {
    assert_eq!(bar_percent(0.0, 100.0), 2.0);
    assert_eq!(bar_percent(1.0, 1000.0), 2.0);
  }

  #[test]
  fn bar_percent_handles_zero_max() {
    assert_eq!(bar_percent(0.0, 0.0), 2.0);
  }
}
