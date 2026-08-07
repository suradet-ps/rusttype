use std::sync::{Arc, Mutex};

use leptos::either::Either;
use leptos::prelude::*;

use engine::{KeyResult, TypingState};

use super::code_display::CodeDisplay;
use super::hidden_input::HiddenInput;
use super::results_summary::ResultsSummary;
use super::stats_bar::StatsBar;

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

#[component]
pub fn TypingSession(snippet: snippets::Snippet) -> impl IntoView {
  let code = snippet.code.clone();
  let initial_state = TypingState::new(&code);
  let initial_target = initial_state.target.clone();

  let state = Arc::new(Mutex::new(initial_state));
  let restart_state = Arc::clone(&state);
  let stats_state = Arc::clone(&state);

  let (target_chars, _set_target) = signal(initial_target);
  let (cursor, set_cursor) = signal(0usize);
  let (error_count, set_error_count) = signal(0usize);
  let (wpm, set_wpm) = signal(0.0f64);
  let (accuracy, set_accuracy) = signal(100.0f64);
  let (completed, set_completed) = signal(false);
  let (session_stats, set_session_stats) = signal(None::<engine::SessionStats>);
  let (wrong_positions, set_wrong_positions) = signal(Vec::<usize>::new());

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

  // The keystroke handler — defined as a plain fn so it can be cloned freely
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
        set_cursor.set(total_chars);
        set_completed.set(true);
        set_session_stats.set(Some(stats.clone()));
        set_wpm.set(stats.wpm);
        set_accuracy.set(stats.accuracy * 100.0);
        set_error_count.set(stats.error_count);
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
      set_wrong_positions.set(Vec::new());
      *lock_state(&restart_state) = TypingState::new(&code);
    }
  };

  let title = snippet.title.clone();
  let language = snippet.language;

  view! {
      <div class="typing-session">
          <div class="session-header">
              <h2 class="snippet-title">{title}</h2>
              <span class="snippet-language">{language.display_name()}</span>
          </div>

          {move || {
              if completed.get() {
                  Either::Left(session_stats.get().map(|stats| {
                      view! { <ResultsSummary stats=stats on_restart=on_restart.clone() /> }
                  }))
              } else {
                  Either::Right(view! {
                      <div class="typing-area">
                          <CodeDisplay
                              code={target_chars.get().into_iter().collect::<String>()}
                              language=language
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
                          <HiddenInput on_key=move |ch| handle_key.run(ch) />
                      </div>
                  })
              }
          }}
      </div>
  }
}
