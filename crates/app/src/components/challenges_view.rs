//! Challenge projects: one card per source project.

use leptos::prelude::*;
use snippets::{Challenge, all_challenges};

use crate::progress::{Progress, stars_for_levels};

#[component]
pub fn ChallengesView(
  on_back: impl Fn() + Clone + Send + Sync + 'static,
  on_open: Callback<&'static str>,
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

          <div class="challenge-cards">
              {groups.into_iter().map(|(project, levels)| {
                  let project_stars = stars_for_levels(&progress, &levels);
                  let project_max = levels.len() as u32 * 3;
                  let license = levels.first().map_or("", |level| level.license);
                  view! {
                      <button
                          class="challenge-card"
                          on:click=move |_| on_open.run(project)
                      >
                          <span class="challenge-title">{project}</span>
                          <span class="challenge-meta">
                              {format!("{} levels · {license}", levels.len())}
                          </span>
                          <span class="challenge-card-foot">
                              <span class="challenge-best">
                                  {format!("{project_stars} / {project_max} ★")}
                              </span>
                              <span class="challenge-practice">"Open"</span>
                          </span>
                      </button>
                  }
              }).collect_view()}
          </div>
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
