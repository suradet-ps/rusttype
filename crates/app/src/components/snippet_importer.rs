//! Import a user-provided Rust snippet: title + paste box.

use leptos::prelude::*;
use snippets::{MAX_SNIPPET_LENGTH, SnippetStore};

use crate::store::LocalSnippetStore;

#[component]
pub fn SnippetImporter(on_saved: Callback<()>, on_cancel: Callback<()>) -> impl IntoView {
  let (title, set_title) = signal(String::new());
  let (code, set_code) = signal(String::new());
  let (error, set_error) = signal(None::<String>);

  let can_save = move || !code.get().trim().is_empty();
  let code_len = move || code.get().chars().count();

  let save = move || {
    let result = LocalSnippetStore::new().add_user_snippet(title.get(), code.get());
    match result {
      Ok(_) => on_saved.run(()),
      Err(err) => set_error.set(Some(err.to_string())),
    }
  };

  view! {
      <div class="snippet-importer card-content">
          <p class="eyebrow">"Add your own"</p>
          <h2 class="section-title">"Import a snippet"</h2>
          <p class="picker-subtitle">
              "Paste any Rust code - it is stored only in this browser (localStorage)."
          </p>

          <div class="form-field">
              <label class="field-label" for="import-title">"Title"</label>
              <input
                  id="import-title"
                  class="text-input"
                  prop:value={move || title.get()}
                  placeholder="Leave blank to use the first line of code"
                  on:input=move |ev| set_title.set(event_target_value(&ev))
              />
          </div>

          <div class="form-field">
              <label class="field-label" for="import-code">"Code"</label>
              <textarea
                  id="import-code"
                  class="text-input code-input"
                  placeholder="fn main() {\n    println!(\"hello\");\n}\n"
                  spellcheck="false"
                  on:input=move |ev| set_code.set(event_target_value(&ev))
              ></textarea>
              <span class="char-count">
                  {move || format!("{} / {} characters", code_len(), MAX_SNIPPET_LENGTH)}
              </span>
          </div>

          {move || {
              error.get().map(|message| view! { <div class="form-error">{message}</div> })
          }}

          <div class="form-actions">
              <button
                  class="btn btn-primary"
                  disabled=move || !can_save()
                  on:click=move |_| save()
              >
                  "Save snippet"
              </button>
              <button class="btn btn-tertiary" on:click=move |_| on_cancel.run(())>
                  "Cancel"
              </button>
          </div>
      </div>
  }
}
