use leptos::either::Either;
use leptos::prelude::*;
use snippets::Snippet;

mod components;
mod history;
mod keyboard;
mod progress;
mod settings;
mod share_card;
mod store;

use components::{HistoryView, SnippetImporter, SnippetPicker, TypingSession};

/// Which screen the app is showing.
#[derive(Clone)]
enum View {
  Picker,
  Importer,
  History,
  Session(Snippet),
}

#[component]
fn App() -> impl IntoView {
  let (view, set_view) = signal(View::Picker);
  let (store_version, set_store_version) = signal(0u32);

  let back_to_picker = move || set_view.set(View::Picker);

  view! {
      <div class="app">
          <header class="nav-bar">
              <div class="nav-brand">
                  <img class="nav-logo" src="favicon.svg" alt="" />
                  <div class="nav-brand-text">
                      <h1 class="nav-title">"RustType"</h1>
                      <p class="nav-subtitle">"Practice typing real code"</p>
                  </div>
              </div>
              {move || matches!(view.get(), View::Picker).then(|| view! {
                  <nav class="nav-actions" aria-label="Sections">
                      <button
                          class="nav-action"
                          on:click=move |_| set_view.set(View::History)
                      >
                          <svg
                              class="nav-action-icon"
                              viewBox="0 0 16 16"
                              fill="none"
                              stroke="currentColor"
                              stroke-width="1.5"
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              aria-hidden="true"
                          >
                              <circle cx="8" cy="8" r="6.25"></circle>
                              <path d="M8 4.5V8l2.5 1.5"></path>
                          </svg>
                          "History"
                      </button>
                  </nav>
              })}
          </header>

          <main class="main-content">
              {move || match view.get() {
                  View::Importer => Either::Left(view! {
                      <SnippetImporter
                          on_saved=Callback::new(move |()| {
                              set_view.set(View::Picker);
                              set_store_version.update(|v| *v += 1);
                          })
                          on_cancel=Callback::new(move |()| set_view.set(View::Picker))
                      />
                  }),
                  View::History => Either::Right(Either::Left(view! {
                      <HistoryView on_back=back_to_picker />
                  })),
                  View::Session(snippet) => Either::Right(Either::Right(Either::Left(view! {
                      <TypingSession snippet=snippet on_back=back_to_picker />
                  }))),
                  View::Picker => Either::Right(Either::Right(Either::Right(view! {
                      <SnippetPicker
                          on_select=Callback::new(move |snippet| set_view.set(View::Session(snippet)))
                          on_import=Callback::new(move |()| set_view.set(View::Importer))
                          on_changed=Callback::new(move |()| set_store_version.update(|v| *v += 1))
                          store_version=store_version.into()
                      />
                  }))),
              }}
          </main>
      </div>
  }
}

pub fn main() {
  console_error_panic_hook::set_once();
  leptos::mount::mount_to_body(App);
}
