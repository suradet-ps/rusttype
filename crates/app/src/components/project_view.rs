//! One challenge project: its levels, laid out as rows.

use leptos::prelude::*;
use snippets::{Challenge, all_challenges};

use crate::progress::{Progress, stars_for_levels, stars_label};

#[component]
pub fn ProjectView(
  project: &'static str,
  on_back: impl Fn() + Clone + Send + Sync + 'static,
  on_start: Callback<Challenge>,
) -> impl IntoView {
  let progress = Progress::load();
  let levels: Vec<&Challenge> = all_challenges()
    .iter()
    .filter(|level| level.project == project)
    .collect();
  let project_stars = stars_for_levels(&progress, &levels);
  let project_max = levels.len() as u32 * 3;
  let license = levels.first().map_or("", |level| level.license);
  let subtitle = format!("{} levels · {license}", levels.len());

  view! {
      <div class="challenges-view">
          <div class="challenges-header">
              <button class="btn btn-tertiary btn-sm" on:click=move |_| on_back()>
                  "Challenges"
              </button>
              <div class="challenges-title-block">
                  <h2 class="section-title">{project}</h2>
                  <span class="challenges-subtitle">{subtitle}</span>
              </div>
              <span class="challenges-total">
                  {format!("{project_stars} / {project_max} ★")}
              </span>
          </div>

          <div class="challenge-rows">
              {levels.into_iter().map(|level| {
                  let best = progress.best(level.id);
                  let stars = stars_label(best.map_or(0, |best| best.stars));
                  let record = best.map_or_else(
                      || "Not attempted".to_string(),
                      |best| format!("{:.1} WPM · {:.1}%", best.wpm, best.accuracy * 100.0),
                  );
                  let challenge = *level;
                  view! {
                      <div class="challenge-row">
                          <span class="challenge-order">
                              {format!("{:02}", level.level)}
                          </span>
                          <div class="challenge-info">
                              <span class="challenge-title">{level.title}</span>
                              <span class="challenge-meta">
                                  {format!("{} · {}", level.source_path, level.license)}
                              </span>
                          </div>
                          <div class="challenge-result">
                              <span class="challenge-stars">{stars}</span>
                              <span class="challenge-best">{record}</span>
                          </div>
                          <button
                              class="btn btn-primary btn-sm"
                              on:click=move |_| on_start.run(challenge)
                          >"Start"</button>
                      </div>
                  }
              }).collect_view()}
          </div>
      </div>
  }
}
