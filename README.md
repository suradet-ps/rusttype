<div align="center">

# RustType

**A code-typing practice app for Rust developers.**

Built with [Leptos](https://github.com/leptos-rs/leptos) (CSR/WASM) + [syntect](https://github.com/trishume/syntect) syntax highlighting.
Practice touch-typing real Rust code with strict-mode correctness, per-character error tracking, and WPM/accuracy stats.

[![CI](https://github.com/user/codetype/actions/workflows/ci.yml/badge.svg)](https://github.com/user/codetype/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

---

## Features

- **Strict Mode** — cursor cannot advance past an incorrect keystroke
- **Auto-Indent** — IDE-style indentation skip after Enter (no manual spaces/tabs)
- **Live Stats** — WPM, accuracy, and error count update in real-time
- **Syntax Highlighting** — powered by `syntect`, code looks like your editor
- **8 Embedded Snippets** — hand-picked Rust code samples ready to practice
- **Error Patterns** — worst mistyped tokens surfaced for targeted drills
- **Zero Dependencies at Runtime** — pure WASM, no backend required

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable, with `wasm32-unknown-unknown` target)
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)

### Setup

```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Install trunk
cargo install trunk

# Run dev server (from repo root)
trunk serve --port 3000
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

## Usage

1. Pick a snippet from the list
2. Click the typing area to focus
3. Type the code — strict mode blocks wrong keystrokes
4. Auto-indent kicks in after each newline
5. Complete the snippet to see your stats (WPM, accuracy, duration)

## Tech Stack

| Concern | Choice |
|---|---|
| UI Framework | Leptos v0.8 (CSR/WASM) |
| Syntax Highlighting | `syntect` |
| Serialization | `serde` + `serde_json` |
| Local Persistence | `localStorage` via `web-sys` |
| Error Handling | `thiserror` (libraries), `anyhow` (app) |
| Build Tooling | Trunk |
| Lint / CI | `cargo fmt`, `cargo clippy -- -D warnings`, `cargo audit` |

## Development

```bash
# Run all checks (fmt, clippy, tests)
cargo fmt --check
cargo clippy -- -D warnings
cargo test

# Run WASM dev server (from repo root)
trunk serve --port 3000
```

## Milestones

| Milestone | Status |
|---|---|
| **M0** — Core typing loop (strict mode, auto-indent, embedded snippets) | Done |
| **M1** — Live stats (WPM, accuracy, progress bar) | Done |
| **M2** — Wrong-keystroke UX (visual flash, shake) | Done |
| **M3** — Indentation correctness (auto-indent tests) | Done |
| **M4** — User-provided snippets (paste + save) | Pending |
| **M5** — Session history (localStorage) | Pending |
| **M6** — Drill mode (worst-token practice) | Pending |
| **M7** — Export results (PNG via Canvas2D) | Pending |

## License

RustType is released under the MIT License.
