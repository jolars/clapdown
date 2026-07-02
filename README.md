# clapdown

[![CI](https://github.com/jolars/clapdown/actions/workflows/ci.yml/badge.svg)](https://github.com/jolars/clapdown/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/clapdown.svg)](https://crates.io/crates/clapdown)
[![docs.rs](https://img.shields.io/docsrs/clapdown)](https://docs.rs/clapdown)

Generate well-structured Markdown documentation from a [`clap`] CLI.

`clapdown` walks a `clap::Command` and renders it to Markdown whose heading
outline tracks command nesting **without ever skipping a level**: the root
command is a top-level heading, each subcommand descends exactly one level, and
a command's `Arguments`/`Options` sections sit one level below the command.
Arguments render as definition lists.

It is inspired by [`clap-markdown`] but fixes that crate's malformed heading
outline (which jumps `h1 -> h2 -> h6` and renders every subcommand as a flat
`h2`) and is built around an output-*flavor* abstraction.

## Usage

Add the crate (typically as a build-dependency, since generating docs from a
`clap::Command` is a build-time concern):

```toml
[build-dependencies]
clapdown = "0.1"
clap = { version = "4", features = ["derive"] }
```

Render from a derived CLI type:

```rust
use clap::CommandFactory;

let options = clapdown::Options::new()
    .base_heading_level(1)
    .footer(false);

let markdown = clapdown::render(&Cli::command(), &options);
```

or go straight from the type:

```rust
let markdown = clapdown::render_from::<Cli>(&clapdown::Options::new());
```

## Options

  | Method                       | Default      | Effect                                     |
  | ---------------------------- | ------------ | ------------------------------------------ |
  | `flavor(Flavor)`             | `Mdbook`     | Target Markdown flavor.                    |
  | `base_heading_level(u8)`     | `1`          | Heading level of the root command.         |
  | `title(impl Into<String>)`   | command name | Override the root heading (Pandoc: title). |
  | `table_of_contents(bool)`    | `false`      | Emit a nested table of contents.           |
  | `footer(bool)`               | `false`      | Emit an attribution footer.                |
  | `aliases(bool)`              | `true`       | Show command aliases.                      |
  | `metadata(bool)`             | `true`       | Emit the Pandoc YAML metadata block.       |
  | `metadata_field(key, value)` | none         | Add a custom Pandoc metadata field.        |

## Flavors

`Flavor::Mdbook` and `Flavor::Pandoc` are implemented; a `CommonMark` flavor is
planned. Both emit [definition lists], which render natively in [mdBook]
(enabled by default) and in Pandoc. The `Flavor` enum is `#[non_exhaustive]`.

`Flavor::Pandoc` additionally prefixes the document with a [YAML metadata block]
carrying the `title` (from `title(...)`, else the command name) plus any
`metadata_field(...)` entries. Because the title lives in the metadata, the root
command's `h1` is omitted from the body to avoid duplicating it. Disable the
block with `metadata(false)`, in which case Pandoc output matches `Mdbook`.

```yaml
---
title: demo
author: Jane Doe
---
```

## License

Licensed under either of MIT or Apache-2.0 at your option.

[`clap`]: https://docs.rs/clap
[`clap-markdown`]: https://github.com/ConnorGray/clap-markdown
[mdBook]: https://rust-lang.github.io/mdBook/
[definition lists]: https://rust-lang.github.io/mdBook/format/markdown.html
[YAML metadata block]: https://pandoc.org/MANUAL.html#extension-yaml_metadata_block
