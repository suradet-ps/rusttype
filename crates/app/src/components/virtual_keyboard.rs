//! On-screen QWERTY keyboard: finger colours + next-key highlight.
//!
//! Renders the physical keyboard (see `crate::keyboard`) with one colour per
//! finger (`{finger.*}` tokens from DESIGN.md) and outlines the key that
//! produces the next target character in `{colors.primary}`. This component
//! is visual-only — input always comes from the physical keyboard.

use leptos::prelude::*;

use crate::keyboard::{
  BOTTOM_ROW, Finger, HOME_ROW, KeyDef, KeyId, NUMBER_ROW, TOP_ROW, key_id_for_char, needs_shift,
};

#[component]
pub fn VirtualKeyboard(next_char: Signal<Option<char>>) -> impl IntoView {
  let next_id = move || next_char.get().and_then(key_id_for_char);
  let next_shift = move || next_char.get().is_some_and(needs_shift);

  view! {
      <div
          class="virtual-keyboard"
          role="img"
          aria-label="QWERTY keyboard layout showing the next key to press"
      >
          <div class="kb-legend">
              {Finger::ALL.into_iter().map(|finger| {
                  view! {
                      <span class="kb-legend-item">
                          <span class={format!("kb-legend-dot {}", finger.class())}></span>
                          {finger.name()}
                      </span>
                  }
              }).collect_view()}
          </div>

          <div class="kb-row">
              {render_keys(&NUMBER_ROW, 0, next_char, next_id).collect_view()}
              {special_key("kb-backspace", None, "Backspace", next_id, next_shift)}
          </div>
          <div class="kb-row">
              {special_key("kb-tab", Some(KeyId::Tab), "Tab", next_id, next_shift)}
              {render_keys(&TOP_ROW, 1, next_char, next_id).collect_view()}
          </div>
          <div class="kb-row">
              {render_keys(&HOME_ROW, 2, next_char, next_id).collect_view()}
              {special_key("kb-enter", Some(KeyId::Enter), "Enter", next_id, next_shift)}
          </div>
          <div class="kb-row">
              {special_key("kb-shift", Some(KeyId::Shift), "Shift", next_id, next_shift)}
              {render_keys(&BOTTOM_ROW, 3, next_char, next_id).collect_view()}
              {special_key("kb-shift", Some(KeyId::Shift), "Shift", next_id, next_shift)}
          </div>
          <div class="kb-row">
              {special_key("kb-space", Some(KeyId::Space), "Space", next_id, next_shift)}
          </div>
      </div>
  }
}

/// Render one row of printable keys with fine-grained highlight classes.
///
/// Highlight state is applied via `class:` directives, so a keystroke only
/// toggles the DOM class of the affected key instead of re-formatting every
/// key label.
fn render_keys(
  row: &'static [KeyDef],
  row_index: usize,
  next_char: Signal<Option<char>>,
  next_id: impl Fn() -> Option<KeyId> + Copy + Send + Sync + 'static,
) -> Vec<impl IntoView> {
  row
    .iter()
    .enumerate()
    .map(move |(col, key)| {
      let base = format!("kb-key {}", key.finger.class());
      let is_current = move || {
        next_id()
          == Some(KeyId::Char {
            row: row_index,
            col,
          })
      };
      let is_shift_target = move || {
        is_current()
          && key
            .shift
            .is_some_and(|symbol| symbol.chars().next() == next_char.get())
      };
      view! {
          <span
              class=base
              class:kb-current=is_current
              class:kb-current-shift=is_shift_target
          >
              {key.shift.map(|symbol| view! { <span class="kb-shift-label">{symbol}</span> })}
              <span class="kb-base-label">{key.base}</span>
          </span>
      }
    })
    .collect()
}

/// Render a special (non-printable) key: Tab, Enter, Shift, Backspace, Space.
///
/// Only keys with a `KeyId` can be highlighted as the next key; both Shift
/// keys highlight whenever the next character needs the Shift modifier.
fn special_key(
  width_class: &'static str,
  id: Option<KeyId>,
  label: &'static str,
  next_id: impl Fn() -> Option<KeyId> + Copy + Send + Sync + 'static,
  next_shift: impl Fn() -> bool + Copy + Send + Sync + 'static,
) -> impl IntoView {
  let base = format!("kb-key kb-special {width_class}");
  let is_current = move || match id {
    Some(KeyId::Shift) => next_shift(),
    Some(id) => next_id() == Some(id),
    None => false,
  };
  view! {
      <span class=base class:kb-current=is_current>{label}</span>
  }
}
