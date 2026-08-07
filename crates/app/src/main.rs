use leptos::either::Either;
use leptos::prelude::*;

mod components;
mod style;

use components::TypingSession;
use snippets::{EmbeddedSnippets, Language};

#[component]
fn App() -> impl IntoView {
  let embedded = EmbeddedSnippets::new();
  let snippets = embedded.list(Some(Language::Rust));
  let (selected, set_selected) = signal(None::<snippets::Snippet>);

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
                  if let Some(s) = selected.get() {
                      Either::Left(view! { <TypingSession snippet=s /> })
                  } else {
                      Either::Right(view! {
                          <div class="snippet-picker">
                              <h2 class="section-title">"Choose a snippet"</h2>
                              <div class="snippet-list">
                                  {snippets.iter().map(|s| {
                                      let id = s.id.clone();
                                      let title = s.title.clone();
                                      let preview: String = s.code.chars().take(80).collect();
                                        view! {
                                            <button
                                                class="snippet-card"
                                                on:click=move |_| {
                                                  let s = snippets::EmbeddedSnippets::new()
                                                      .list(Some(Language::Rust))
                                                      .into_iter()
                                                      .find(|s| s.id == id)
                                                      .expect("invariant: snippet id exists");
                                                  set_selected.set(Some(s));
                                              }
                                          >
                                              <span class="snippet-card-title">{title}</span>
                                              <code class="snippet-card-preview">{preview}</code>
                                          </button>
                                      }
                                  }).collect_view()}
                              </div>
                          </div>
                      })
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
