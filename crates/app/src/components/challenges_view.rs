//! Challenge levels: curated real-world code with star goals.

use leptos::prelude::*;
use snippets::{Challenge, all_challenges};

use crate::progress::{Progress, stars_label};

#[component]
pub fn ChallengesView(
  on_back: impl Fn() + Clone + Send + Sync + 'static,
  on_start: Callback<Challenge>,
) -> impl IntoView {
  let progress = Progress::load();
  let total_stars = progress.total_stars();
  let max_stars = all_challenges().len() as u32 * 3;
  let groups = group_by_project(all_challenges());

  view! {
      <div class="challenges-view">
          <div class="challenges-header">
              <button class="btn btn-tertiary btn-sm" on:click=move |_| on_back()>
                  "All snippets"
              </button>
              <h2 class="section-title">"Challenges"</h2>
              <span class="challenges-total">
                  {format!("{total_stars} / {max_stars} ★")}
              </span>
          </div>

          {groups.into_iter().map(|(project, levels)| {
              view! {
                  <section class="challenge-group">
                      <h3 class="challenge-project">{project}</h3>
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
                                          {format!(
                                              "{} · {} · {}",
                                              level.project,
                                              level.source_path,
                                              level.license,
                                          )}
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
                  </section>
              }
          }).collect_view()}
      </div>
  }
}

/// Group levels by project, keeping level order inside each group.
fn group_by_project(levels: &[Challenge]) -> Vec<(&str, Vec<&Challenge>)> {
  let mut groups: Vec<(&str, Vec<&Challenge>)> = Vec::new();
  for level in levels {
    match groups.last_mut() {
      Some((project, list)) if *project == level.project => list.push(level),
      _ => groups.push((level.project, vec![level])),
    }
  }
  groups
}

#[cfg(test)]
mod tests {
  use super::*;

  fn level(id: &'static str, project: &'static str) -> Challenge {
    Challenge {
      id,
      title: "Title",
      project,
      source_path: "src/lib.rs",
      license: "MIT",
      level: 1,
      accuracy_goal: 0.97,
      wpm_goal: 30.0,
      code: "fn main() {}\n",
    }
  }

  #[test]
  fn groups_contiguous_projects() {
    let levels = [
      level("a", "one"),
      level("b", "one"),
      level("c", "two"),
      level("d", "one"),
    ];
    let groups = group_by_project(&levels);
    assert_eq!(groups.len(), 3);
    assert_eq!(groups[0].0, "one");
    assert_eq!(groups[0].1.len(), 2);
    assert_eq!(groups[1].0, "two");
    assert_eq!(groups[2].0, "one");
  }

  #[test]
  fn empty_input_has_no_groups() {
    assert!(group_by_project(&[]).is_empty());
  }
}
