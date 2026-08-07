//! User settings persisted to localStorage.

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

/// Versioned localStorage key for app settings.
const SETTINGS_KEY: &str = "rusttype:settings:v1";

/// App settings.
///
/// `#[serde(default)]` makes the schema growable: unknown or missing fields
/// fall back to [`Default`] values instead of failing to load.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
  /// Show the on-screen finger-coloured keyboard during a session.
  pub show_virtual_keyboard: bool,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      show_virtual_keyboard: true,
    }
  }
}

impl Settings {
  /// Load settings from localStorage, falling back to defaults on any
  /// failure (storage unavailable, corrupt JSON, key absent). Never panics.
  pub fn load() -> Self {
    let Some(raw) = storage()
      .and_then(|storage| storage.get_item(SETTINGS_KEY).ok())
      .flatten()
    else {
      return Self::default();
    };
    serde_json::from_str(&raw).unwrap_or_else(|error| {
      log_failure(&error.to_string());
      Self::default()
    })
  }

  /// Persist settings to localStorage. Failures are logged, never panicked.
  pub fn save(&self) {
    let Ok(json) = serde_json::to_string(self) else {
      log_failure("serialization failed");
      return;
    };
    let Some(storage) = storage() else { return };
    if let Err(error) = storage.set_item(SETTINGS_KEY, &json) {
      log_failure(&format!("{error:?}"));
    }
  }
}

/// Access browser localStorage without panicking.
fn storage() -> Option<web_sys::Storage> {
  web_sys::window()?.local_storage().ok()?.take()
}

/// Log a settings failure so it is never silent.
fn log_failure(reason: &str) {
  web_sys::console::log_1(&JsValue::from_str(&format!(
    "rusttype: failed to load settings ({reason}); using defaults"
  )));
}
