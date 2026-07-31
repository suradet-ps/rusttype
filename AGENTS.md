# AGENTS.md — RustType

A code-typing practice web app for Rust developers, built with Leptos v0.8 (CSR/WASM). Practice touch-typing real code with syntax highlighting, strict-mode correctness, and per-language WPM/accuracy tracking.

This document is the implementation contract for AI coding agents (and humans) working on this repo. It references `DESIGN.md` for all visual tokens (colors, spacing, typography) — do not restate design values here; look them up by token name in `DESIGN.md`.

Rust-wide conventions (error handling, lint policy, CI) are defined once in the shared `AGENTS-RUST.md` and apply to every crate in this workspace unless explicitly overridden below.

---

## 1. Project Goal

Let a developer paste or pick a code snippet and type it out under **strict mode**: the cursor cannot advance past an incorrect keystroke. The app tracks WPM, accuracy, and per-character error patterns, and surfaces which symbols/tokens (e.g. `::`, `->`, `{}`) the user mistypes most often.

Non-goals (out of scope for v1): multiplayer/races, mobile native app, server-side snippet moderation.

---

## 2. Workspace Architecture (ports-lite)

```
codetype/
├── Cargo.toml                 # workspace root
├── AGENTS.md                  # this file
├── AGENTS-RUST.md             # shared Rust conventions (symlink or copy from other projects)
├── DESIGN.md                  # design tokens (colors, type scale, spacing) — single source of truth
├── crates/
│   ├── app/                   # Leptos CSR UI: components, routing, top-level state
│   ├── engine/                # typing state machine, WPM/accuracy calc, keystroke event log
│   └── snippets/               # Snippet model, embedded snippet loader, user-provided snippet storage
└── snippets-data/
    └── rust/*.rs               # raw embedded snippet source files (include_str! targets)
```

**Dependency direction:** `app` depends on `engine` and `snippets`. `engine` and `snippets` do not depend on `app` or on each other except where `snippets::Snippet` is the input type to `engine::TypingState::new`. Neither `engine` nor `snippets` may depend on Leptos — they must be pure Rust, unit-testable without a browser.

---

## 3. Tech Stack

| Concern | Choice | Notes |
|---|---|---|
| UI framework | Leptos v0.8, CSR/WASM | no SSR in v1 |
| Syntax highlighting | `syntect` | reuse config approach from CodeShot project |
| Serialization | `serde` + `serde_json` | for snippet storage and keystroke logs |
| Local persistence | `localStorage` via `web-sys` | user-provided snippets, session stats, settings |
| Error handling | `thiserror` (library crates), `anyhow` (app-level only) | per AGENTS-RUST.md — no `unwrap`/`expect` in non-test code |
| Build tooling | `trunk` | CSR bundling |
| Lint/CI | `cargo fmt`, `cargo clippy -- -D warnings`, `cargo audit` | per AGENTS-RUST.md, SHA-pinned GitHub Actions |

---

## 4. Core Domain Types

### 4.1 `engine` crate

```rust
pub struct TypingState {
    pub target: Vec<char>,
    pub cursor: usize,
    pub error_count: usize,
    pub started_at: Option<Instant>,
    pub finished_at: Option<Instant>,
    pub keystrokes: Vec<KeystrokeEvent>,
}

pub struct KeystrokeEvent {
    pub expected: char,
    pub actual: char,
    pub correct: bool,
    pub at: Instant,
}

pub enum KeyResult {
    Correct { cursor: usize },
    Wrong { expected: char, actual: char },
    Completed { stats: SessionStats },
}

pub struct SessionStats {
    pub wpm: f64,
    pub accuracy: f64,          // correct_keystrokes / total_keystrokes
    pub error_count: usize,
    pub duration: Duration,
    pub worst_tokens: Vec<(String, usize)>, // top mistyped substrings, for drill mode later
}
```

Rules the engine must enforce (strict mode):
- `cursor` only advances on `Correct`. A `Wrong` keystroke never mutates `cursor`.
- Auto-indent: after a correct newline keystroke, the engine automatically skips leading whitespace (spaces/tabs) on the next line — IDE-style. Skipped indentation is free: it records no keystrokes and does not affect WPM or accuracy. Newlines themselves and all inline whitespace remain real target characters the user must type.
- `error_count` counts *keystroke attempts*, not distinct wrong characters (a user hammering the wrong key 5 times before getting it right counts 5 errors at that position).
- WPM formula: standard `(correct_chars / 5) / minutes_elapsed`, computed only from `started_at` (first keystroke) to `finished_at` (last correct keystroke that completes the target).

### 4.2 `snippets` crate

```rust
pub enum Language { Rust, Python, JavaScript, TypeScript, Go, C, Cpp }

pub enum SnippetSource {
    Embedded,
    UserProvided,
}

pub struct Snippet {
    pub id: String,
    pub language: Language,
    pub source: SnippetSource,
    pub title: String,
    pub code: String,   // raw, untrimmed — indentation is significant
}

pub trait SnippetStore {
    fn list(&self, language: Option<Language>) -> Vec<Snippet>;
    fn add_user_snippet(&mut self, title: String, language: Language, code: String) -> Result<Snippet, SnippetError>;
    fn remove(&mut self, id: &str) -> Result<(), SnippetError>;
}
```

- `Embedded` snippets are loaded via `include_str!` over files in `snippets-data/rust/` (and future per-language folders) enumerated by a build script — never fetched over the network.
- `UserProvided` snippets are persisted to `localStorage` under a versioned key (e.g. `rusttype:snippets:v1`) and validated (non-empty, reasonable max length — see `SnippetError::TooLong`) before storage.
- `SnippetError` uses `thiserror` with variants: `Empty`, `TooLong(usize)`, `StorageUnavailable`, `SerializationFailed`.

---

## 5. UI Component Structure (`app` crate)

```
<TypingSession snippet=Snippet>
  ├── <CodeDisplay />     // renders syntect-highlighted code; overlays cursor marker + wrong-char flash
  ├── <HiddenInput />     // captures on:keydown; kept focused; not visibly rendered as a text box
  └── <StatsBar />        // live WPM, accuracy, error_count
<SnippetPicker />          // browse embedded snippets by language
<SnippetImporter />        // paste/upload user code, choose language, save to SnippetStore
<ResultsSummary stats=SessionStats />
```

Key implementation constraints:
- Keystroke capture uses `on:keydown` on a focused hidden `<input>`/`<textarea>`, not a global window listener — avoids IME and browser-shortcut collisions.
- `CodeDisplay` re-renders highlighting spans only for the affected region on each keystroke, not the whole snippet, to keep large snippets responsive.
- All colors, fonts, and spacing in `CodeDisplay` / `StatsBar` reference `DESIGN.md` tokens by name (e.g. `--color-correct`, `--color-error`, `--font-mono`) — do not hardcode hex values in components.

---

## 6. Milestones

### M0 — Core typing loop (embedded snippets only)
- `engine::TypingState` + `KeyResult` implemented and unit-tested (correct/wrong/completed paths, no-unwrap).
- `snippets` crate with `Embedded` source only, 5–10 hand-picked Rust snippets in `snippets-data/rust/`.
- `CodeDisplay` renders syntect-highlighted target with cursor position and current-char highlight.
- `HiddenInput` wired to `on:keydown`, strict-mode blocking confirmed working end-to-end.

### M1 — Live stats
- `StatsBar` shows running WPM and accuracy while typing (not just at completion).
- `SessionStats` computed correctly on `Completed`, including duration and error_count.
- Manual QA: verify WPM against a stopwatch for at least one full snippet.

### M2 — Wrong-keystroke UX polish
- Visual flash/shake on the mistyped character (CSS only, tokens from `DESIGN.md`).
- Distinguish "wrong character" from "wrong whitespace/newline" in the UI, since these are the most common frustration point in strict mode.

### M3 — Indentation correctness
- Explicit test coverage in `engine` for auto-indent: multi-level indentation (tabs vs spaces, nested blocks), blank lines, trailing whitespace, and confirming skipped indentation records no keystrokes.
- Snippet author guidelines documented (in this file, Section 8) for how embedded snippets should be formatted (`cargo fmt` output only, no trailing whitespace).

### M4 — User-provided snippets
- `SnippetImporter` component: paste box + language selector + save.
- `localStorage`-backed `SnippetStore` implementation with `SnippetError` handling surfaced in the UI (no silent failures).
- `SnippetPicker` merges embedded + user snippets, filterable by language and source.

### M5 — Session history
- Persist `SessionStats` per completed session to `localStorage` (key namespaced separately from snippets, e.g. `rusttype:history:v1`).
- Simple history view: WPM/accuracy over time, filterable by language.
- Data model versioned so future Supabase sync (see Section 9) can migrate cleanly.

### M6 — Drill mode
- Use `SessionStats::worst_tokens` across history to generate a focused practice snippet emphasizing the user's most-mistyped substrings (e.g. `::`, `->`, `{}`).
- New `SnippetSource::Generated` variant for these synthetic drills.

### M7 — Export results
- Render a shareable result card (stats + snippet excerpt) to PNG via HTML Canvas2D, reusing the export approach from the CodeShot project.
- No server round-trip required — client-side render and download only.

---

## 7. Testing Requirements

- `engine`: unit tests for every `KeyResult` branch, WPM formula edge cases (zero elapsed time, single-character snippet), and indentation-sensitive sequences.
- `snippets`: unit tests for `SnippetStore` validation errors (`Empty`, `TooLong`) and round-trip serialization.
- No `unwrap`/`expect` outside `#[cfg(test)]` blocks, per `AGENTS-RUST.md`.
- `cargo clippy -- -D warnings` and `cargo fmt --check` must pass in CI before merge.

---

## 8. Snippet Authoring Guidelines (embedded)

- Embedded snippets must be valid, `cargo fmt`-formatted source (for Rust) with no trailing whitespace and Unix line endings.
- Keep embedded snippets between 10–40 lines — long enough to be meaningful practice, short enough for a single focused session.
- Prefer snippets that exercise a mix of common Rust punctuation (`::`, `->`, `<>`, `{}`, `?`, `&`) rather than prose-heavy comments.

---

## 9. Future / Explicitly Deferred

- Supabase sync for cross-device history (pattern already established in other projects; defer until M5 data model is stable).
- Additional languages beyond Rust for embedded snippets (Python/JS/Go/etc.) — data model already supports it via `Language` enum, just needs content.
- Multiplayer/race mode — out of scope, not planned.

---

## 10. References

- `DESIGN.md` — all visual design tokens (color, typography, spacing, layout grid). Look up by token name; do not duplicate values here.
- `AGENTS-RUST.md` — shared Rust conventions (error handling, lint policy, CI security hardening) applying to all crates in this workspace.
