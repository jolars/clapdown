# Agent guidance

This file provides guidance to Agents when working with code in this repository.

`clapdown` is a Rust library that walks a `clap::Command` and renders it to
Markdown. See `README.md` for the public-facing overview.

## Commands

```sh
cargo test --all-features                    # run all tests
cargo test --all-features headings_never     # run a single test by name
cargo fmt --all --check                      # formatting check (CI gate)
cargo clippy --all-features --all-targets    # lints (CI gate, warnings are errors)
```

CI runs test, fmt, and clippy with `RUSTFLAGS="-D warnings"`. Toolchain is
pinned to Rust 1.88 (`rust-toolchain.toml`); the crate uses edition 2024.
`clippy` and `rustfmt` also run as pre-commit git hooks under devenv.

### Snapshot tests

Rendering is covered by an [`insta`] snapshot in
`tests/snapshots/render__full_document_snapshot.snap`. When output intentionally
changes, review and accept with `cargo insta review` (or `cargo insta accept`).

## Architecture

Three modules under `src/`:

- **`lib.rs`** --- public API surface. Re-exports `Options` and `Flavor`, and
  exposes free functions `render(&Command, &Options)` and
  `render_from::<C: CommandFactory>(&Options)`.
- **`options.rs`** --- the `Options` builder (chained setters, then `.render()`)
  and the `Flavor` enum. `Flavor` is `#[non_exhaustive]`; `Mdbook` and `Pandoc`
  exist, with `CommonMark` planned, so match arms must stay open. `Pandoc`
  differs only in a leading YAML metadata block (see below); the `metadata` and
  `metadata_fields` options drive it.
- **`render.rs`** --- the whole rendering engine, private to the crate. `render`
  recurses through `render_command`, carrying a `path` (Vec of command-name
  segments) and a `depth`. Flavor-specific formatting is isolated in small
  helpers (`write_heading`, `definition`, `arg_term`, `slug`) so new flavors can
  be added with minimal churn.

### Rendering conventions

- Descriptions prefer the long about, falling back to the short about; argument
  help prefers long help over short. Mirror this precedence for new fields.
- The auto-generated `help` and `version` args, and any `hide`-set item, are
  filtered out.
- Arguments render as mdBook/Pandoc **definition lists** (`term` line, then
  `:   definition`). `definition` handles multi-block, multi-line bodies.
- The `Pandoc` flavor prepends a YAML metadata block (`write_yaml_metadata`)
  with the `title` (the `title` option, else the command name) plus any
  `metadata_fields`. When the block is emitted (`emits_metadata`), the root
  command's `h1` is skipped in `render_command` so the title is not duplicated.
  Values are quoted only when needed (`yaml_scalar`/`is_plain_scalar`).
- Output is normalized to a single trailing newline.

### Test fixture

`tests/common/mod.rs` defines a derive-based `Cli` fixture deliberately
exercising the tricky cases: a global flag, nested subcommands, a variadic
positional, default values, and a `ValueEnum` with per-variant docs. Extend this
fixture when adding features rather than writing bespoke commands per test.

[`insta`]: https://insta.rs/
