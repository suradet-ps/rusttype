use std::sync::{Arc, Mutex};

use leptos::either::Either;
use leptos::prelude::*;

use engine::{KeyResult, TypingState};

use super::code_display::CodeDisplay;
use super::results_summary::ResultsSummary;
use super::stats_bar::StatsBar;

/// Get current time in milliseconds using `performance.now()`.
fn now_ms() -> f64 {
  web_sys::window()
    .expect("no window")
    .performance()
    .expect("no performance")
    .now()
}

/// Ignore non-printable / modifier keys.
fn is_printable_key(key: &str) -> bool {
  !matches!(
    key,
    "Shift"
      | "Control"
      | "Alt"
      | "Meta"
      | "Escape"
      | "ArrowUp"
      | "ArrowDown"
      | "ArrowLeft"
      | "ArrowRight"
      | "Home"
      | "End"
      | "PageUp"
      | "PageDown"
      | "Insert"
      | "Delete"
      | "Backspace"
      | "F1"
      | "F2"
      | "F3"
      | "F4"
      | "F5"
      | "F6"
      | "F7"
      | "F8"
      | "F9"
      | "F10"
      | "F11"
      | "F12"
  )
}

#[component]
pub fn TypingSession(snippet: snippets::Snippet) -> impl IntoView {
  let code = snippet.code.clone();
  let initial_state = TypingState::new(&code);
  let initial_target = initial_state.target.clone();

  let state = Arc::new(Mutex::new(initial_state));

  let (target, _set_target) = signal(initial_target);
  let (cursor, set_cursor) = signal(0usize);
  let (error_count, set_error_count) = signal(0usize);
  let (wpm, set_wpm) = signal(0.0f64);
  let (accuracy, set_accuracy) = signal(100.0f64);
  let (completed, set_completed) = signal(false);
  let (session_stats, set_session_stats) = signal(None::<engine::SessionStats>);
  let (wrong_positions, set_wrong_positions) = signal(Vec::<usize>::new());

  let total_chars = target.get_untracked().len();

  // The keystroke handler — defined as a plain fn so it can be cloned freely
  let handle_key = {
    let state = state.clone();
    Callback::new(move |ch: char| {
      if completed.get_untracked() {
        return;
      }

      let now = now_ms();
      let result = state
        .lock()
        .expect("invariant: single-threaded WASM, no concurrent panics")
        .process_key_at(ch, now);

      match result {
        KeyResult::Correct { cursor: new_cursor } => {
          set_cursor.set(new_cursor);
          set_wrong_positions.update(|v| v.clear());
          let keystrokes = state
            .lock()
            .expect("invariant: single-threaded WASM, no concurrent panics")
            .keystrokes
            .clone();
          let correct_count = keystrokes.iter().filter(|k| k.correct).count();
          let total_keystrokes = keystrokes.len();
          let acc = if total_keystrokes > 0 {
            correct_count as f64 / total_keystrokes as f64 * 100.0
          } else {
            100.0
          };
          let elapsed_ms = {
            let s = state
              .lock()
              .expect("invariant: single-threaded WASM, no concurrent panics");
            now - s.started_at_ms.unwrap_or(now)
          };
          let minutes = elapsed_ms / 60_000.0;
          let w = if minutes > 0.0 {
            (correct_count as f64 / 5.0) / minutes
          } else {
            0.0
          };
          set_wpm.set(w);
          set_accuracy.set(acc);
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
          let keystrokes = state
            .lock()
            .expect("invariant: single-threaded WASM, no concurrent panics")
            .keystrokes
            .clone();
          let correct_count = keystrokes.iter().filter(|k| k.correct).count();
          let total_keystrokes = keystrokes.len();
          let acc = if total_keystrokes > 0 {
            correct_count as f64 / total_keystrokes as f64 * 100.0
          } else {
            100.0
          };
          set_accuracy.set(acc);
        }
        KeyResult::Completed { stats } => {
          set_cursor.set(total_chars);
          set_completed.set(true);
          set_session_stats.set(Some(stats.clone()));
          set_wpm.set(stats.wpm);
          set_accuracy.set(stats.accuracy * 100.0);
          set_error_count.set(stats.error_count);
        }
      }
    })
  };

  let progress = move || {
    let c = cursor.get() as f64;
    let t = total_chars as f64;
    if t > 0.0 { c / t } else { 0.0 }
  };

  let on_restart = {
    let state = state.clone();
    let code = snippet.code.clone();
    move || {
      set_cursor.set(0);
      set_error_count.set(0);
      set_wpm.set(0.0);
      set_accuracy.set(100.0);
      set_completed.set(false);
      set_session_stats.set(None);
      set_wrong_positions.set(Vec::new());
      *state
        .lock()
        .expect("invariant: single-threaded WASM, no concurrent panics") = TypingState::new(&code);
    }
  };

  let title = snippet.title.clone();
  let language_name = snippet.language.display_name().to_string();

  view! {
      <div class="typing-session">
          <div class="session-header">
              <h2 class="snippet-title">{title}</h2>
              <span class="snippet-language">{language_name}</span>
          </div>

          {move || {
              if completed.get() {
                  Either::Left(session_stats.get().map(|stats| {
                      view! { <ResultsSummary stats=stats on_restart=on_restart.clone() /> }
                  }))
              } else {
                   let key_handler = handle_key;
                  Either::Right(view! {
                      <div
                          class="typing-area"
                          tabindex="0"
                          on:keydown=move |ev: web_sys::KeyboardEvent| {
                              ev.prevent_default();
                              let key = ev.key();
                              // Enter → newline, Tab → tab
                              if key == "Enter" {
                                  key_handler.run('\n');
                                  return;
                              }
                              if key == "Tab" {
                                  key_handler.run('\t');
                                  return;
                              }
                              if !is_printable_key(&key) {
                                  return;
                              }
                              let chars: Vec<char> = key.chars().collect();
                              if let Some(&ch) = chars.first()
                                  && chars.len() == 1
                              {
                                  key_handler.run(ch);
                              }
                          }
                      >
                          <CodeDisplay
                              target=target.get()
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
                      </div>
                  })
              }
          }}
      </div>
  }
}
