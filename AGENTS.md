# AGENTS.md - RustType

A code-typing practice web app for Rust developers, built with Leptos v0.8 (CSR/WASM). Practice touch-typing real Rust code with syntax highlighting, strict-mode correctness, and WPM/accuracy tracking.

This document is the implementation contract for AI coding agents (and humans) working on this repo. It references `docs/DESIGN.md` for all visual tokens (colors, spacing, typography) - do not restate design values here; look them up by token name in `docs/DESIGN.md`.

Rust-wide conventions (error handling, lint policy, CI) are defined once in the shared `docs/AGENTS-RUST.md` and apply to every crate in this workspace unless explicitly overridden below.

---

## 1. Project Goal

Let a developer paste or pick a code snippet and type it out under **strict mode**: the cursor cannot advance past an incorrect keystroke. The app tracks WPM, accuracy, and per-character error patterns, and surfaces which symbols/tokens (e.g. `::`, `->`, `{}`) the user mistypes most often.

Non-goals (out of scope for v1): multiplayer/races, mobile native app, server-side snippet moderation.

---

## 2. Workspace Architecture (ports-lite)

```
rusttype/
├── Cargo.toml                 # workspace root
├── AGENTS.md                  # this file
├── docs/
│   ├── DESIGN.md              # design tokens (colors, type scale, spacing) - single source of truth
│   └── AGENTS-RUST.md         # shared Rust conventions (symlink or copy from other projects)
├── crates/
│   ├── app/                   # Leptos CSR UI: components, routing, top-level state
│   ├── engine/                # typing state machine, WPM/accuracy calc, keystroke event log
│   └── snippets/               # Snippet model, embedded snippet loader, user-provided snippet storage
└── snippets-data/
    └── rust/*.rs               # raw embedded snippet source files (include_str! targets)
```

**Dependency direction:** `app` depends on `engine` and `snippets`. `engine` and `snippets` do not depend on `app` or on each other except where `snippets::Snippet` is the input type to `engine::TypingState::new`. Neither `engine` nor `snippets` may depend on Leptos - they must be pure Rust, unit-testable without a browser.

---

## 3. Tech Stack

| Concern | Choice | Notes |
|---|---|---|
| UI framework | Leptos v0.8, CSR/WASM | no SSR in v1 |
| Syntax highlighting | `syntect` | reuse config approach from CodeShot project |
| Serialization | `serde` + `serde_json` | for snippet storage and keystroke logs |
| Local persistence | `localStorage` via `web-sys` | user-provided snippets, session stats, settings |
| Error handling | `thiserror` (library crates), `anyhow` (app-level only) | per docs/AGENTS-RUST.md - no `unwrap`/`expect` in non-test code |
| Build tooling | `trunk` | CSR bundling |
| Lint/CI | `cargo fmt`, `cargo clippy -- -D warnings`, `cargo audit` | per docs/AGENTS-RUST.md, SHA-pinned GitHub Actions |

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
- Auto-indent: after a correct newline keystroke, the engine automatically skips leading whitespace (spaces/tabs) on the next line - IDE-style. Skipped indentation is free: it records no keystrokes and does not affect WPM or accuracy. Newlines themselves and all inline whitespace remain real target characters the user must type.
- `error_count` counts *keystroke attempts*, not distinct wrong characters (a user hammering the wrong key 5 times before getting it right counts 5 errors at that position).
- WPM formula: standard `(correct_chars / 5) / minutes_elapsed`, computed only from `started_at` (first keystroke) to `finished_at` (last correct keystroke that completes the target).

### 4.2 `snippets` crate

RustType is Rust-only by design - snippets carry no language field.

```rust
pub enum SnippetSource {
    Embedded,
    UserProvided,
}

pub struct Snippet {
    pub id: String,
    pub source: SnippetSource,
    pub title: String,
    pub code: String,   // raw, untrimmed - indentation is significant
}

pub trait SnippetStore {
    fn list(&self) -> Vec<Snippet>;
    fn add_user_snippet(&mut self, title: String, code: String) -> Result<Snippet, SnippetError>;
    fn remove(&mut self, id: &str) -> Result<(), SnippetError>;
}
```

- `Embedded` snippets are loaded via `include_str!` over files in `snippets-data/rust/` enumerated at compile time - never fetched over the network. All embedded snippets are Rust.
- `UserProvided` snippets are persisted to `localStorage` under a versioned key (e.g. `rusttype:snippets:v1`) and validated (non-empty, reasonable max length - see `SnippetError::TooLong`) before storage. An empty title is derived from the first line of code.
- `SnippetError` uses `thiserror` with variants: `Empty`, `TooLong(usize)`, `StorageUnavailable`, `SerializationFailed`, `NotFound`.
- `validate_user_snippet(title, code)` is the single validation entry point shared by every store implementation; it returns a snippet with an empty `id` that the store fills before persisting.

---

## 5. UI Component Structure (`app` crate)

```
<TypingSession snippet=Snippet>
  ├── <CodeDisplay />     // renders syntect-highlighted code; overlays cursor marker + wrong-char flash
  ├── <HiddenInput />     // captures on:keydown; kept focused; not visibly rendered as a text box
  ├── <StatsBar />        // live WPM, accuracy, error_count
  └── <VirtualKeyboard /> // optional on-screen QWERTY: finger colours + next-key highlight (settings toggle)
<SnippetPicker />          // browse embedded + user snippets by source
<SnippetImporter />        // paste Rust code, save to SnippetStore
<ResultsSummary stats=SessionStats />
```

Key implementation constraints:
- Keystroke capture uses `on:keydown` on a focused hidden `<input>`/`<textarea>`, not a global window listener - avoids IME and browser-shortcut collisions.
- `CodeDisplay` re-renders highlighting spans only for the affected region on each keystroke, not the whole snippet, to keep large snippets responsive.
- Auto-scroll: the current-char span is anchored at 20% from the left edge of the `.code-display` scroll container and clamped vertically so the typed line stays visible (the pane scrolls both axes internally; the page itself never jumps). Implemented as an `Effect` in `TypingSession` (where the cursor signal lives), keyed off the `rusttype-current-char` DOM id.
- `VirtualKeyboard` layout data lives in `crate::keyboard` (pure Rust, unit-tested): QWERTY rows, per-finger colour classes (`finger-lp`…`finger-rp`, `{finger.*}` tokens), and `key_id_for_char` mapping target characters to physical keys. QWERTY is a physical-keyboard concern, not a language model - Rust-only stays intact.
- All colors, fonts, and spacing in `CodeDisplay` / `StatsBar` / `VirtualKeyboard` reference `docs/DESIGN.md` tokens by name (e.g. `--color-correct`, `--color-error`, `--font-mono`) - do not hardcode hex values in components.

---

## 6. Milestones

### M0 - Core typing loop (embedded snippets only)
- `engine::TypingState` + `KeyResult` implemented and unit-tested (correct/wrong/completed paths, no-unwrap).
- `snippets` crate with `Embedded` source only, 5-10 hand-picked Rust snippets in `snippets-data/rust/`.
- `CodeDisplay` renders syntect-highlighted target with cursor position and current-char highlight.
- `HiddenInput` wired to `on:keydown`, strict-mode blocking confirmed working end-to-end.

### M1 - Live stats
- `StatsBar` shows running WPM and accuracy while typing (not just at completion).
- `SessionStats` computed correctly on `Completed`, including duration and error_count.
- Manual QA: verify WPM against a stopwatch for at least one full snippet.

### M2 - Wrong-keystroke UX polish
- Visual flash/shake on the mistyped character (CSS only, tokens from `docs/DESIGN.md`).
- Distinguish "wrong character" from "wrong whitespace/newline" in the UI, since these are the most common frustration point in strict mode.

### M3 - Indentation correctness
- Explicit test coverage in `engine` for auto-indent: multi-level indentation (tabs vs spaces, nested blocks), blank lines, trailing whitespace, and confirming skipped indentation records no keystrokes.
- Snippet author guidelines documented (in this file, Section 8) for how embedded snippets should be formatted (`cargo fmt` output only, no trailing whitespace).

### M4 - User-provided snippets
- `SnippetImporter` component: paste box + title + save.
- `localStorage`-backed `SnippetStore` implementation with `SnippetError` handling surfaced in the UI (no silent failures).
- `SnippetPicker` merges embedded + user snippets, filterable by source.

### M4.5 - Typing ergonomics
- The whole page never scrolls: `.app` is exactly `100vh` with `overflow: hidden`. The typing session is a fixed "board" - header top, `VirtualKeyboard` pinned at the very bottom, and the `.code-display` pane fills the remaining space and scrolls internally (both axes) - long snippets never push the keyboard off-screen. Picker/importer/results views scroll internally too; only the code pane ever moves.
- Auto-scroll: the code pane scrolls internally (both axes) and is only moved when the cursor char leaves the comfortable band - horizontally it anchors at 20% from the left edge, vertically it keeps a 35% look-ahead margin below the cursor line (at least 24 px) so the next lines are readable before they are typed. The code always starts at the top of the pane; short snippets simply leave the space below empty. Implemented as an `Effect` in `TypingSession` keyed off the `rusttype-current-char` DOM id.
- Keyboard shortcuts: `Tab` restarts the session (unless the next target character is a literal tab, which indentation requires - then it types it); `Esc` returns to the snippet picker. Handled in `HiddenInput` via an `on_shortcut` callback, not global listeners.
- `VirtualKeyboard`: optional on-screen QWERTY showing per-finger colours and highlighting the key for the next required character. Layout + key mapping live in `crate::keyboard` (pure Rust, unit-tested). QWERTY is a physical-keyboard concern, not a language model - Rust-only stays intact.
- The keyboard is a user preference persisted in `localStorage` under `rusttype:settings:v1` (default on), versioned and `#[serde(default)]`-growable via `crate::settings`.
- Wrong-keystroke animations respect `prefers-reduced-motion` (flash/shake disabled).

### M5 - Session history
- Persist `SessionStats` per completed session to `localStorage` (key namespaced separately from snippets, e.g. `rusttype:history:v1`).
- Simple history view: WPM/accuracy over time.
- Data model versioned so future Supabase sync (see Section 9) can migrate cleanly.

### M6 - Drill mode
- Use `SessionStats::worst_tokens` across history to generate a focused practice snippet emphasizing the user's most-mistyped substrings (e.g. `::`, `->`, `{}`).
- New `SnippetSource::Generated` variant for these synthetic drills.

### M7 - Export results
- Render a shareable result card (stats + snippet excerpt) to PNG via HTML Canvas2D, reusing the export approach from the CodeShot project.
- No server round-trip required - client-side render and download only.

### M8 - Challenge levels
- Curated levels are real Rust excerpts from real projects, copied verbatim under `challenges/<project>/` and embedded at compile time (see `challenges/README.md`). The first sets are six excerpts each from ripgrep (MIT), serde (MIT OR Apache-2.0), hashbrown (MIT OR Apache-2.0) and proptest (MIT OR Apache-2.0).
- Every level carries attribution (project, source path, license) and goals, shown to the user.
- No gating: all levels are open, stars are the collectible. One star for finishing, two at the accuracy goal (`0.97`), three at the level's WPM goal.
- Best result per level persists in `rusttype:progress:v1`; the session log stays in `rusttype:history:v1`.
- UI: a separate Challenges view (levels grouped by project, stars and best attempt) with a nav entry; the results screen shows the earned stars for a challenge session.

---

## 7. Testing Requirements

- `engine`: unit tests for every `KeyResult` branch, WPM formula edge cases (zero elapsed time, single-character snippet), and indentation-sensitive sequences.
- `snippets`: unit tests for `SnippetStore` validation errors (`Empty`, `TooLong`) and round-trip serialization.
- No `unwrap`/`expect` outside `#[cfg(test)]` blocks, per `docs/AGENTS-RUST.md`.
- `cargo clippy -- -D warnings` and `cargo fmt --check` must pass in CI before merge.

---

## 8. Snippet Authoring Guidelines (embedded)

- Embedded snippets must be valid, `cargo fmt`-formatted source (for Rust) with no trailing whitespace and Unix line endings.
- Keep embedded snippets between 10-40 lines - long enough to be meaningful practice, short enough for a single focused session.
- Prefer snippets that exercise a mix of common Rust punctuation (`::`, `->`, `<>`, `{}`, `?`, `&`) rather than prose-heavy comments.

---

## 9. Future / Explicitly Deferred

- Supabase sync for cross-device history (pattern already established in other projects; defer until M5 data model is stable).
- Importable challenge packs (the `challenges/` layout and metadata are the format to grow into).
- RustType is deliberately Rust-only. Reintroducing other languages (Python/JS/Go/etc.) would require restoring a language model on `Snippet` - a deliberate, non-trivial decision.
- Multiplayer/race mode - out of scope, not planned.

---

## 10. References

- `docs/DESIGN.md` - all visual design tokens (color, typography, spacing, layout grid). Look up by token name; do not duplicate values here.
- `docs/AGENTS-RUST.md` - shared Rust conventions (error handling, lint policy, CI security hardening) applying to all crates in this workspace.
