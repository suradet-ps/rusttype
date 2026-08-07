use leptos::either::Either;
use leptos::prelude::*;
use snippets::Snippet;

mod components;
mod store;

use components::{SnippetImporter, SnippetPicker, TypingSession};

#[component]
fn App() -> impl IntoView {
  let (selected, set_selected) = signal(None::<Snippet>);
  let (importing, set_importing) = signal(false);
  let (store_version, set_store_version) = signal(0u32);

  let back_to_picker = move || {
    set_selected.set(None);
    set_importing.set(false);
  };

  view! {
      <div class="app">
          <header class="nav-bar">
              <div class="nav-brand">
                  <img class="nav-logo" src="favicon.svg" alt="RustType logo" />
                  <h1 class="nav-title">"RustType"</h1>
              </div>
              <p class="nav-subtitle">"Practice typing real code"</p>
          </header>

          <main class="main-content">
              {move || {
                  if importing.get() {
                      Either::Left(view! {
                          <SnippetImporter
                              on_saved=Callback::new(move |()| {
                                  set_importing.set(false);
                                  set_store_version.update(|v| *v += 1);
                              })
                              on_cancel=Callback::new(move |()| set_importing.set(false))
                          />
                      })
                  } else if let Some(snippet) = selected.get() {
                      Either::Right(Either::Left(view! {
                          <TypingSession snippet=snippet on_back=back_to_picker />
                      }))
                  } else {
                      Either::Right(Either::Right(view! {
                          <SnippetPicker
                              on_select=Callback::new(move |snippet| set_selected.set(Some(snippet)))
                              on_import=Callback::new(move |()| set_importing.set(true))
                              on_changed=Callback::new(move |()| set_store_version.update(|v| *v += 1))
                              store_version=store_version.into()
                          />
                      }))
                  }
              }}
          </main>
      </div>
  }
}

pub fn main() {
  console_error_panic_hook::set_once();
  leptos::mount::mount_to_body(App);
}
