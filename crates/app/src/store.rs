//! localStorage-backed [`SnippetStore`] implementation for user snippets.

use snippets::{Snippet, SnippetError, SnippetStore, validate_user_snippet};
use wasm_bindgen::JsValue;

/// Versioned localStorage key for user-provided snippets.
const STORAGE_KEY: &str = "rusttype:snippets:v1";

/// Snippet store persisted in browser localStorage.
///
/// Stateless: every method reads or writes the full snippet list from
/// localStorage, so a fresh instance can be created for any call.
pub struct LocalSnippetStore;

impl LocalSnippetStore {
  /// Create a new store handle.
  pub fn new() -> Self {
    Self
  }

  /// Access browser localStorage, mapping absence/failure to a domain error.
  fn storage() -> Result<web_sys::Storage, SnippetError> {
    web_sys::window()
      .ok_or(SnippetError::StorageUnavailable)?
      .local_storage()
      .map_err(|_| SnippetError::StorageUnavailable)?
      .ok_or(SnippetError::StorageUnavailable)
  }

  /// Load all persisted user snippets.
  fn load(&self) -> Result<Vec<Snippet>, SnippetError> {
    let storage = Self::storage()?;
    let raw = storage
      .get_item(STORAGE_KEY)
      .map_err(|_| SnippetError::StorageUnavailable)?;
    match raw {
      None => Ok(Vec::new()),
      Some(json) => serde_json::from_str(&json).map_err(|_| SnippetError::SerializationFailed),
    }
  }

  /// Persist the full user snippet list.
  fn save(&self, snippets: &[Snippet]) -> Result<(), SnippetError> {
    let storage = Self::storage()?;
    let json = serde_json::to_string(snippets).map_err(|_| SnippetError::SerializationFailed)?;
    storage
      .set_item(STORAGE_KEY, &json)
      .map_err(|_| SnippetError::StorageUnavailable)
  }

  /// Log a load failure so corruption is never silent.
  fn log_load_failure(error: &SnippetError) {
    web_sys::console::log_1(&JsValue::from_str(&format!(
      "rusttype: failed to load user snippets: {error}"
    )));
  }
}

impl Default for LocalSnippetStore {
  fn default() -> Self {
    Self::new()
  }
}

impl SnippetStore for LocalSnippetStore {
  fn list(&self) -> Vec<Snippet> {
    match self.load() {
      Ok(snippets) => snippets,
      Err(error) => {
        Self::log_load_failure(&error);
        Vec::new()
      }
    }
  }

  fn add_user_snippet(&mut self, title: String, code: String) -> Result<Snippet, SnippetError> {
    let mut snippet = validate_user_snippet(title, code)?;
    let mut all = self.load()?;
    snippet.id = format!("user-{}", js_sys::Date::now() as u64);
    all.push(snippet.clone());
    self.save(&all)?;
    Ok(snippet)
  }

  fn remove(&mut self, id: &str) -> Result<(), SnippetError> {
    let mut all = self.load()?;
    let len_before = all.len();
    all.retain(|s| s.id != id);
    if all.len() == len_before {
      return Err(SnippetError::NotFound);
    }
    self.save(&all)
  }
}
