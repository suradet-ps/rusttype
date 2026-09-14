# RustType

```
   ██████╗ ██╗   ██╗███████╗████████╗████████╗██╗   ██╗██████╗ ███████╗
   ██╔══██╗██║   ██║██╔════╝╚══██╔══╝╚══██╔══╝╚██╗ ██╔╝██╔══██╗██╔════╝
   ██████╔╝██║   ██║███████╗   ██║      ██║    ╚████╔╝ ██████╔╝█████╗
   ██╔══██╗██║   ██║╚════██║   ██║      ██║     ╚██╔╝  ██╔══██╗██╔══╝
   ██║  ██║╚██████╔╝███████║   ██║      ██║      ██║   ██████╔╝███████╗
   ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝      ╚═╝      ╚═╝   ╚═════╝ ╚══════╝
```

---

## ◆ PULSE

The keyboard does not negotiate. Every keystroke is judged, and the cursor
never advances past a mistake. RustType turns real Rust source into a
discipline drill - your hands learn the language before your mind pretends
it already knows it.

| 0 | 1 | 2 | 3 | | 4 | 5 | 6 | 7 |
|---|---|---|---|--|---|---|---|---|
| ▣ | ▣ | ▣ | ▣ | | ▣ | ▣ | ▣ | ▣ |

*Core loop, live stats, wrong-key UX, indentation rigor, user snippets, M4.5
typing ergonomics, session history, drill mode, export, and the first
challenge set are sealed.*

> Compiled with Leptos v0.8 (CSR/WASM), highlighted by `syntect`, judged by
> a strict-mode engine that lives in pure Rust - no browser, no excuses.
>
> **suradet-ps**, artifact keeper

---

## ◆ IGNITION

One sequence. Nothing more.

```
⟫ rustup target add wasm32-unknown-unknown
⟫ cargo install trunk
⟫ trunk serve --port 3000
```

Open [http://localhost:3000](http://localhost:3000). The first snippet is
already waiting.

<details>
<summary>Prerequisites</summary>

- [Rust](https://rustup.rs/) (stable toolchain, pinned in `rust-toolchain.toml`)
- [Trunk](https://trunkrs.dev/) - the bundler, installed above

</details>

---

## ◆ ANATOMY

- **Judges** - `engine::TypingState` locks the cursor against the expected
  character. A wrong key is recorded, never forgiven, never skipped.
- **Indents** - after a correct newline, leading whitespace on the next line
  is swallowed for free. No keystrokes, no WPM credit, no mercy on the
  newline itself.
- **Measures** - WPM from first keystroke to last correct one; accuracy is
  attempts against truth. The top mistyped tokens are surfaced for drills.
- **Remembers** - user snippets, settings, and session history live in
  versioned `localStorage` keys. Nothing leaves your machine; there is no
  backend to betray you.
- **Shares** - the finished session renders to a PNG card on an off-screen
  canvas at 2x, using the same design tokens as the UI. One click, no server,
  no screenshot.
- **Challenges** - curated levels cut from real projects, six each from
  ripgrep, serde, hashbrown, proptest, tokio, axum and leptos to start. Every
  level carries its source path and license, and pays up to three stars for
  speed and accuracy. No gates; only stars.

---

## ◆ RITUALS

**The core ceremony** - every session is the same shape:

1. Pick a snippet. `Tab` at any time restarts it; `Esc` returns to the picker.
2. Click the board to arm it. The next required character is lit.
3. Type. Wrong keystrokes flash and are held back - strict mode never blinks.
4. Cross the final character and the results surface: WPM, accuracy,
   duration, and the tokens that tripped you.

**The ritual of the wrong key:**

```
expected:  :
you sent:  ;
engine:    [blocked]  error_count: 1  cursor: still here
```

The cursor stays. The count grows. The muscle memory is yours to build.

---

## ◆ ECHOES

**Where this artifact is heading**

```
2026 ▸ M4   user-provided snippets ────────── sealed
     ▸ M4.5 typing ergonomics ─────────────── sealed
     ▸ M5   session history ───────────────── sealed
     ▸ M6   drill mode on worst tokens ────── sealed
     ▸ M7   export results to PNG ─────────── sealed
     ▸ M8   real-code challenge levels ────── sealed
```

*Every milestone is sealed - the rest is practice.*

**Raising the artifact** - issues and pull requests are welcomed under the
rules in `AGENTS.md`. The engine is pure Rust: `cargo fmt --check`,
`cargo clippy -- -D warnings`, and `cargo test` must pass before anything
merges. No `unwrap` outside tests. No exceptions.

**Status** - CI runs every commit: [workflows](.github/workflows).

---

```
  ─────────────────────────────────────────────────────────
   The keyboard does not negotiate.
   Neither should your practice.
  ─────────────────────────────────────────────────────────
```

RustType is released under the [MIT License](LICENSE).