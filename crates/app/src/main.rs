use leptos::either::EitherOf5;
use leptos::prelude::*;
use snippets::{Challenge, Snippet};

mod components;
mod history;
mod keyboard;
mod progress;
mod settings;
mod share_card;
mod store;

use components::{ChallengesView, HistoryView, SnippetImporter, SnippetPicker, TypingSession};

/// Which screen the app is showing.
#[derive(Clone)]
enum View {
  Picker,
  Importer,
  History,
  Challenges,
  Session {
    snippet: Snippet,
    challenge: Option<Challenge>,
  },
}

#[component]
fn App() -> impl IntoView {
  let (view, set_view) = signal(View::Picker);
  let (store_version, set_store_version) = signal(0u32);

  let back_to_picker = move || set_view.set(View::Picker);
  // A challenge session returns to the challenge list; everything else
  // returns to the picker.
  let back_from_session = move || {
    if matches!(
      view.get_untracked(),
      View::Session {
        challenge: Some(_),
        ..
      }
    ) {
      set_view.set(View::Challenges);
    } else {
      set_view.set(View::Picker);
    }
  };

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
                          on:click=move |_| set_view.set(View::Challenges)
                      >
                          <svg
                              class="nav-action-icon"
                              viewBox="0 0 16 16"
                              fill="none"
                              stroke="currentColor"
                              stroke-width="1.5"
                              stroke-linejoin="round"
                              aria-hidden="true"
                          >
                              <path d="M8 2.5l1.7 3.4 3.8.6-2.75 2.7.65 3.8L8 11.2l-3.4 1.8.65-3.8L2.5 6.5l3.8-.6L8 2.5z"></path>
                          </svg>
                          "Challenges"
                      </button>
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
                  View::Importer => EitherOf5::A(view! {
                      <SnippetImporter
                          on_saved=Callback::new(move |()| {
                              set_view.set(View::Picker);
                              set_store_version.update(|v| *v += 1);
                          })
                          on_cancel=Callback::new(move |()| set_view.set(View::Picker))
                      />
                  }),
                  View::History => EitherOf5::B(view! {
                      <HistoryView on_back=back_to_picker />
                  }),
                  View::Challenges => EitherOf5::C(view! {
                      <ChallengesView
                          on_back=back_to_picker
                          on_start=Callback::new(move |challenge: Challenge| {
                              set_view.set(View::Session {
                                  snippet: challenge.snippet(),
                                  challenge: Some(challenge),
                              });
                          })
                      />
                  }),
                  View::Session { snippet, challenge } => EitherOf5::D(view! {
                      <TypingSession
                          snippet=snippet
                          challenge=challenge
                          on_back=back_from_session
                      />
                  }),
                  View::Picker => EitherOf5::E(view! {
                      <SnippetPicker
                          on_select=Callback::new(move |snippet| {
                              set_view.set(View::Session {
                                  snippet,
                                  challenge: None,
                              });
                          })
                          on_import=Callback::new(move |()| set_view.set(View::Importer))
                          on_changed=Callback::new(move |()| set_store_version.update(|v| *v += 1))
                          store_version=store_version.into()
                      />
                  }),
              }}
          </main>
      </div>
  }
}

pub fn main() {
  console_error_panic_hook::set_once();
  leptos::mount::mount_to_body(App);
}
