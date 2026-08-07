use leptos::prelude::*;

/// Non-typing shortcuts surfaced by [`HiddenInput`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shortcut {
  /// Restart the current snippet (`Tab`).
  Restart,
  /// Leave the session (`Escape`).
  Back,
}

/// Invisible text input that captures keystrokes.
///
/// Keystrokes are captured via `on:keydown` on this focused, visually hidden
/// `<input>` — not a global window listener — avoiding IME and browser
/// shortcut collisions (see AGENTS.md §5). Enter is translated to `\n` so it
/// types a real target character. `Tab` and `Escape` are reported via
/// `on_shortcut` so the session can decide (restart vs. type a literal tab).
#[component]
pub fn HiddenInput(
  on_key: impl Fn(char) + Clone + 'static,
  on_shortcut: impl Fn(Shortcut) + Clone + 'static,
) -> impl IntoView {
  view! {
      <input
          class="hidden-input"
          type="text"
          autocomplete="off"
          spellcheck="false"
          autocapitalize="off"
          autofocus="true"
          on:keydown=move |ev: web_sys::KeyboardEvent| {
              ev.prevent_default();
              let key = ev.key();
              if key == "Enter" {
                  on_key('\n');
                  return;
              }
              if key == "Tab" {
                  on_shortcut(Shortcut::Restart);
                  return;
              }
              if key == "Escape" {
                  on_shortcut(Shortcut::Back);
                  return;
              }
              if !is_printable_key(&key) {
                  return;
              }
              let mut chars = key.chars();
              if let (Some(ch), None) = (chars.next(), chars.next()) {
                  on_key(ch);
              }
          }
      />
  }
}

/// Ignore non-printable / modifier keys.
fn is_printable_key(key: &str) -> bool {
  !matches!(
    key,
    "Shift"
      | "Control"
      | "Alt"
      | "Meta"
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
