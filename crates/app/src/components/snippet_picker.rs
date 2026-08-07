//! Browse embedded + user snippets, filterable by source.

use leptos::either::Either;
use leptos::prelude::*;
use snippets::{EmbeddedSnippets, Snippet, SnippetSource, SnippetStore};

use crate::store::LocalSnippetStore;

#[component]
pub fn SnippetPicker(
  on_select: Callback<Snippet>,
  on_import: Callback<()>,
  on_changed: Callback<()>,
  store_version: Signal<u32>,
) -> impl IntoView {
  let (source_filter, set_source_filter) = signal(None::<SnippetSource>);
  let (error, set_error) = signal(None::<String>);

  let items = Memo::new(move |_| {
    store_version.get();
    let source = source_filter.get();
    let mut all = EmbeddedSnippets::new().list();
    all.extend(LocalSnippetStore::new().list());
    all.sort_by(|a, b| a.title.cmp(&b.title));
    all
      .into_iter()
      .filter(|s| source.is_none_or(|src| s.source == src))
      .collect::<Vec<_>>()
  });

  view! {
      <div class="snippet-picker">
          <div class="picker-hero">
              <p class="eyebrow">"Practice"</p>
              <h2 class="section-title">"Choose a Rust snippet"</h2>
              <p class="picker-subtitle">
                  "Pick an embedded classic or import your own code — strict mode will keep you honest."
              </p>
          </div>

          <div class="filter-bar">
              <div class="filter-group">
                  <span class="filter-label">"Source"</span>
                  <button
                      class=move || if source_filter.get().is_none() { "filter-chip active" } else { "filter-chip" }
                      on:click=move |_| set_source_filter.set(None)
                  >"All"</button>
                  <button
                      class=move || if source_filter.get() == Some(SnippetSource::Embedded) { "filter-chip active" } else { "filter-chip" }
                      on:click=move |_| set_source_filter.set(Some(SnippetSource::Embedded))
                  >"Embedded"</button>
                  <button
                      class=move || if source_filter.get() == Some(SnippetSource::UserProvided) { "filter-chip active" } else { "filter-chip" }
                      on:click=move |_| set_source_filter.set(Some(SnippetSource::UserProvided))
                  >"My snippets"</button>
              </div>
          </div>

          <div class="picker-actions">
              <button class="btn btn-primary" on:click=move |_| on_import.run(())>
                  "Import your own snippet"
              </button>
          </div>

          {move || {
              let items = items.get();
              if items.is_empty() {
                  Either::Left(view! {
                      <div class="empty-state">
                          <p>"No snippets match these filters."</p>
                          {if source_filter.get().is_some() {
                              Either::Left(view! {
                                  <button class="btn-text" on:click=move |_| set_source_filter.set(None)>
                                      "Clear filters"
                                  </button>
                              })
                          } else {
                              Either::Right(view! {
                                  <button class="btn-text" on:click=move |_| on_import.run(())>
                                      "Import one"
                                  </button>
                              })
                          }}
                      </div>
                  })
              } else {
                  Either::Right(view! {
                      <div class="snippet-list">
                          {items.into_iter().map(|s| {
                              let is_user = s.source == SnippetSource::UserProvided;
                              let id = s.id.clone();
                              let snippet = s.clone();
                              let line_count = s.code.lines().count();
                              let char_count = s.code.chars().count();
                              view! {
                                  <div class="card-wrap">
                                      <button class="snippet-card" on:click=move |_| on_select.run(snippet.clone())>
                                          <span class="snippet-card-title">{s.title.clone()}</span>
                                          <span class="card-badges">
                                              <span class="badge">
                                                  {if is_user { "My snippet" } else { "Embedded" }}
                                              </span>
                                          </span>
                                          <pre class="snippet-card-preview">{s.code.clone()}</pre>
                                          <span class="card-footer">
                                              <span class="card-meta">
                                                  {line_count}" lines · "{char_count}" chars"
                                              </span>
                                              <span class="card-practice">"Practice"</span>
                                          </span>
                                      </button>
                                      {is_user.then(|| {
                                          view! {
                                              <button
                                                  class="card-remove"
                                                  title="Remove this snippet"
                                                  on:click=move |ev| {
                                                      ev.prevent_default();
                                                      ev.stop_propagation();
                                                      match LocalSnippetStore::new().remove(&id) {
                                                          Ok(()) => {
                                                              set_error.set(None);
                                                              on_changed.run(());
                                                          }
                                                          Err(err) => set_error.set(Some(err.to_string())),
                                                      }
                                                  }
                                              >"Remove"</button>
                                          }
                                      })}
                                  </div>
                              }
                          }).collect_view()}
                      </div>
                  })
              }
          }}

          {move || {
              error.get().map(|message| view! {
                  <div class="form-error">"Could not remove snippet: " {message}</div>
              })
          }}
      </div>
  }
}
