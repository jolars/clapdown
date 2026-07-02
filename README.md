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

let markdown = clapdown::Options::new()
    .base_heading_level(1)
    .footer(false)
    .render(&Cli::command());
```

or go straight from the type:

```rust
let markdown = clapdown::render_from::<Cli>(&clapdown::Options::new());
```

## Options

  | Method                     | Default      | Effect                             |
  | -------------------------- | ------------ | ---------------------------------- |
  | `flavor(Flavor)`           | `Mdbook`     | Target Markdown flavor.            |
  | `base_heading_level(u8)`   | `1`          | Heading level of the root command. |
  | `title(impl Into<String>)` | command name | Override the root heading text.    |
  | `table_of_contents(bool)`  | `false`      | Emit a nested table of contents.   |
  | `footer(bool)`             | `false`      | Emit an attribution footer.        |
  | `aliases(bool)`            | `true`       | Show command aliases.              |

## Flavors

Only `Flavor::Mdbook` is implemented today. It emits [definition lists], which
render natively in [mdBook] (enabled by default) and in Pandoc. The `Flavor`
enum is `#[non_exhaustive]`; `CommonMark` and `Pandoc` flavors are planned.

## License

Licensed under either of MIT or Apache-2.0 at your option.

[`clap`]: https://docs.rs/clap
[`clap-markdown`]: https://github.com/ConnorGray/clap-markdown
[mdBook]: https://rust-lang.github.io/mdBook/
[definition lists]: https://rust-lang.github.io/mdBook/format/markdown.html
