use leptos::either::Either;
use leptos::prelude::*;
use snippets::Snippet;

mod components;
mod history;
mod keyboard;
mod settings;
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
                  <img class="nav-logo" src="favicon.svg" alt="RustType logo" />
                  <h1 class="nav-title">"RustType"</h1>
              </div>
              <p class="nav-subtitle">"Practice typing real code"</p>
              {move || matches!(view.get(), View::Picker).then(|| view! {
                  <div class="nav-actions">
                      <button
                          class="btn btn-tertiary btn-sm"
                          on:click=move |_| set_view.set(View::History)
                      >"History"</button>
                  </div>
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
