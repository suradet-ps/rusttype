use std::sync::{Arc, Mutex};

use leptos::either::Either;
use leptos::prelude::*;

use engine::{KeyResult, TypingState};

use super::code_display::CURRENT_CHAR_ID;
use super::code_display::CodeDisplay;
use super::hidden_input::HiddenInput;
use super::hidden_input::Shortcut;
use super::results_summary::ResultsSummary;
use super::stats_bar::StatsBar;
use super::virtual_keyboard::VirtualKeyboard;
use crate::history::{History, SessionRecord};
use crate::progress::Progress;
use crate::settings::Settings;

/// Get current time in milliseconds using `performance.now()`.
fn now_ms() -> f64 {
  web_sys::window()
    .expect("invariant: browser environment always provides window")
    .performance()
    .expect("invariant: browser environment always provides performance.now")
    .now()
}

/// Lock the shared typing state, panicking only on a poisoned mutex.
fn lock_state(state: &Arc<Mutex<TypingState>>) -> std::sync::MutexGuard<'_, TypingState> {
  state
    .lock()
    .expect("invariant: single-threaded WASM, no concurrent panics")
}

/// Fraction of the code pane kept visible below the cursor line, so the next
/// lines can be read before they are typed.
const LOOK_AHEAD_FRACTION: f64 = 0.35;

/// Minimum space kept above the cursor line.
const TOP_MARGIN: f64 = 24.0;

/// Vertical scroll offset that keeps the cursor line inside the comfortable
/// band: at least [`TOP_MARGIN`] below the top of the pane, and the look-ahead
/// margin above the bottom.
fn next_scroll_top(scroll_top: f64, top: f64, bottom: f64, height: f64) -> f64 {
  let look_ahead = (height * LOOK_AHEAD_FRACTION).max(TOP_MARGIN);
  if top < TOP_MARGIN {
    (scroll_top + top - TOP_MARGIN).max(0.0)
  } else if bottom + look_ahead > height {
    (scroll_top + bottom + look_ahead - height).max(0.0)
  } else {
    scroll_top
  }
}

#[component]
pub fn TypingSession(
  snippet: snippets::Snippet,
  challenge: Option<snippets::Challenge>,
  on_back: impl Fn() + Clone + Send + Sync + 'static,
) -> impl IntoView {
  let code = snippet.code.clone();
  let initial_state = TypingState::new(&code);
  let initial_target = initial_state.target.clone();

  let state = Arc::new(Mutex::new(initial_state));
  let restart_state = Arc::clone(&state);
  let stats_state = Arc::clone(&state);
  let record_snippet = snippet.clone();
  let results_snippet = snippet.clone();

  let (target_chars, _set_target) = signal(initial_target);
  let (cursor, set_cursor) = signal(0usize);
  let (error_count, set_error_count) = signal(0usize);
  let (wpm, set_wpm) = signal(0.0f64);
  let (accuracy, set_accuracy) = signal(100.0f64);
  let (completed, set_completed) = signal(false);
  let (session_stats, set_session_stats) = signal(None::<engine::SessionStats>);
  let (earned_stars, set_earned_stars) = signal(None::<u8>);
  let (wrong_positions, set_wrong_positions) = signal(Vec::<usize>::new());
  let (show_keyboard, set_show_keyboard) = signal(Settings::load().show_virtual_keyboard);

  let total_chars = target_chars.get_untracked().len();

  let set_live_stats = move || {
    let s = lock_state(&stats_state);
    let correct_count = s.keystrokes.iter().filter(|k| k.correct).count();
    let total_keystrokes = s.keystrokes.len();
    let acc = if total_keystrokes > 0 {
      correct_count as f64 / total_keystrokes as f64 * 100.0
    } else {
      100.0
    };
    let now = now_ms();
    let elapsed_ms = now - s.started_at_ms.unwrap_or(now);
    let minutes = elapsed_ms / 60_000.0;
    let w = if minutes > 0.0 {
      (correct_count as f64 / 5.0) / minutes
    } else {
      0.0
    };
    set_wpm.set(w);
    set_accuracy.set(acc);
  };

  // The keystroke handler - defined as a plain fn so it can be cloned freely
  let handle_key = Callback::new(move |ch: char| {
    if completed.get_untracked() {
      return;
    }

    let result = lock_state(&state).process_key_at(ch, now_ms());

    match result {
      KeyResult::Correct { cursor: new_cursor } => {
        set_cursor.set(new_cursor);
        set_wrong_positions.update(|v| v.clear());
        set_live_stats();
      }
      KeyResult::Wrong {
        expected: _,
        actual: _,
      } => {
        set_error_count.update(|e| *e += 1);
        set_wrong_positions.update(|v| {
          v.clear();
          v.push(cursor.get_untracked());
        });
        set_live_stats();
      }
      KeyResult::Completed { stats } => {
        if let Some(challenge) = &challenge {
          let stars = challenge.stars(stats.wpm, stats.accuracy);
          let mut progress = Progress::load();
          progress.record(
            challenge.id,
            stars,
            stats.wpm,
            stats.accuracy,
            js_sys::Date::now(),
          );
          progress.save();
          set_earned_stars.set(Some(stars));
        }
        set_cursor.set(total_chars);
        set_completed.set(true);
        set_session_stats.set(Some(stats.clone()));
        set_wpm.set(stats.wpm);
        set_accuracy.set(stats.accuracy * 100.0);
        set_error_count.set(stats.error_count);

        let mut history = History::load();
        history.push(SessionRecord::new(
          &record_snippet,
          &stats,
          js_sys::Date::now(),
        ));
        history.save();
      }
    }
  });

  let progress = move || {
    let c = cursor.get() as f64;
    let t = total_chars as f64;
    if t > 0.0 { c / t } else { 0.0 }
  };

  let on_restart = {
    let code = snippet.code.clone();
    move || {
      set_cursor.set(0);
      set_error_count.set(0);
      set_wpm.set(0.0);
      set_accuracy.set(100.0);
      set_completed.set(false);
      set_session_stats.set(None);
      set_earned_stars.set(None);
      set_wrong_positions.set(Vec::new());
      *lock_state(&restart_state) = TypingState::new(&code);
    }
  };

  let toggle_keyboard = move || {
    let next = !show_keyboard.get_untracked();
    set_show_keyboard.set(next);
    Settings {
      show_virtual_keyboard: next,
    }
    .save();
  };

  // The character that still needs typing, for the keyboard overlay.
  let next_char: Signal<Option<char>> = Memo::new(move |_| {
    let c = cursor.get();
    target_chars.get_untracked().get(c).copied()
  })
  .into();

  // Tab restarts (unless the next target character is a literal tab, which
  // real indentation requires); Escape leaves the session.
  let on_shortcut = {
    let on_restart = on_restart.clone();
    let on_back = on_back.clone();
    move |sc: Shortcut| match sc {
      Shortcut::Restart => {
        let c = cursor.get_untracked();
        if c < total_chars && target_chars.get_untracked()[c] == '\t' {
          handle_key.run('\t');
        } else {
          on_restart();
        }
      }
      Shortcut::Back => on_back(),
    }
  };

  // Keep the current character in view while typing long snippets. The code
  // pane scrolls internally (both axes); we only move it when the cursor char
  // leaves the comfortable band: horizontally it anchors at 20% from the left
  // edge, vertically it keeps a look-ahead margin below the cursor so the next
  // lines are readable before they are typed.
  Effect::new(move |_| {
    let _ = cursor.get();
    let Some(element) = document().get_element_by_id(CURRENT_CHAR_ID) else {
      return;
    };
    let Ok(Some(container)) = element.closest(".code-display") else {
      return;
    };
    let rect = element.get_bounding_client_rect();
    let box_rect = container.get_bounding_client_rect();
    let width = container.client_width() as f64;
    let height = container.client_height() as f64;

    let left = rect.left() - box_rect.left();
    let right = rect.right() - box_rect.left();
    if left < 0.0 || right > width {
      let target = ((container.scroll_left() as f64) + left - width * 0.2).max(0.0) as i32;
      container.set_scroll_left(target);
    }

    let top = rect.top() - box_rect.top();
    let bottom = rect.bottom() - box_rect.top();
    let target = next_scroll_top(container.scroll_top() as f64, top, bottom, height);
    container.set_scroll_top(target as i32);
  });

  let title = snippet.title.clone();

  view! {
      <div class="typing-session">
          <div class="session-header">
              <button class="btn btn-tertiary btn-sm" on:click=move |_| on_back()>
                  "All snippets"
              </button>
              <h2 class="snippet-title">{title}</h2>
              <button class="btn btn-tertiary btn-sm" on:click=move |_| toggle_keyboard()>
                  {move || if show_keyboard.get() { "Hide keyboard" } else { "Show keyboard" }}
              </button>
          </div>

          {move || {
              if completed.get() {
                  Either::Left(session_stats.get().map(|stats| {
                      view! {
                          <ResultsSummary
                              stats=stats
                              snippet=results_snippet.clone()
                              stars=earned_stars.get()
                              on_restart=on_restart.clone()
                          />
                      }
                  }))
              } else {
                  Either::Right(view! {
                      <div class="typing-area">
                          <CodeDisplay
                              code={target_chars.get().into_iter().collect::<String>()}
                              cursor=cursor.get()
                              errors=wrong_positions.get()
                          />
                          <StatsBar
                              wpm=wpm.get()
                              accuracy=accuracy.get()
                              error_count=error_count.get()
                              progress=progress()
                          />
                          <p class="hint">"Click here and start typing"</p>
                          {move || {
                              show_keyboard.get().then(|| view! { <VirtualKeyboard next_char=next_char /> })
                          }}
                          <HiddenInput
                              on_key=move |ch| handle_key.run(ch)
                              on_shortcut=on_shortcut.clone()
                          />
                      </div>
                  })
              }
          }}
      </div>
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn cursor_inside_the_band_keeps_the_scroll() {
    assert_eq!(next_scroll_top(100.0, 200.0, 226.0, 600.0), 100.0);
  }

  #[test]
  fn cursor_near_the_bottom_leaves_look_ahead() {
    // A 600 px pane keeps 210 px (35%) below the line: the bottom lands at 390.
    assert_eq!(next_scroll_top(0.0, 380.0, 406.0, 600.0), 16.0);
  }

  #[test]
  fn cursor_above_the_band_scrolls_up() {
    assert_eq!(next_scroll_top(200.0, 10.0, 36.0, 600.0), 186.0);
  }

  #[test]
  fn scroll_never_goes_negative() {
    assert_eq!(next_scroll_top(0.0, -50.0, -24.0, 600.0), 0.0);
  }

  #[test]
  fn short_panes_still_keep_a_margin() {
    // look_ahead = max(200 * 0.35, 24) = 70.
    assert_eq!(next_scroll_top(0.0, 100.0, 126.0, 200.0), 0.0);
    assert_eq!(next_scroll_top(0.0, 120.0, 146.0, 200.0), 16.0);
  }
}
