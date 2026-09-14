# Challenges

Curated challenge levels for RustType: real Rust code, copied from real
projects. Each file in this directory is one level.

## Layout

```
challenges/
└── <project>/
    └── <NN>-<slug>.rs    # level excerpt, LF line endings, no metadata header
```

## Adding a level

1. Copy the excerpt from the source repository, unmodified except for:
   - line endings normalized to LF,
   - a uniform dedent so the first line starts at column 0,
   - trailing blank lines removed.
2. Keep it between 10 and 40 lines and record the exact source path.
3. Only use permissively licensed sources (MIT, Apache-2.0, BSD). The project
   name, source path and license are shown to the user, so attribution
   travels with every level.
4. Register the entry in `crates/snippets/src/challenge.rs` with title,
   project, source path, license, level number and goals.

## Current sets

| Project | Levels | License | Source |
|---|---|---|---|
| ripgrep | 6 | MIT | https://github.com/BurntSushi/ripgrep |
| serde | 6 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |

The ripgrep excerpts are pinned to revision `3fce3b5` (ignore-0.4.33,
2026-08-04) and the serde excerpts to revision `a874a1b` (2026-08-24) - the
checkouts they were copied from.
