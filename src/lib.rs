//! Generate well-structured Markdown documentation from a [`clap`] CLI.
//!
//! `clapdown` walks a [`clap::Command`] and renders it to Markdown whose
//! heading outline tracks command nesting without ever skipping a level: the
//! root command is a top-level heading, each subcommand descends one level, and
//! a command's `Arguments`/`Options` sections sit one level below the command.
//! Arguments render as definition lists.
//!
//! # Example
//!
//! ```
//! use clap::Command;
//!
//! let cmd = Command::new("greet")
//!     .about("Greet someone")
//!     .arg(clap::Arg::new("name").help("Who to greet"));
//!
//! let markdown = clapdown::render(&cmd, &clapdown::Options::new());
//! assert!(markdown.contains("# `greet`"));
//! ```
//!
//! Use [`render_from`] to go straight from a derived CLI type:
//!
//! ```
//! use clap::Parser;
//!
//! #[derive(Parser)]
//! #[command(name = "greet", about = "Greet someone")]
//! struct Cli {
//!     /// Who to greet
//!     name: String,
//! }
//!
//! let markdown = clapdown::render_from::<Cli>(&clapdown::Options::new());
//! assert!(markdown.contains("# `greet`"));
//! ```

mod options;
mod render;

pub use options::{Flavor, Options};

use clap::{Command, CommandFactory};

/// Render `cmd` to a Markdown string using `opts`.
///
/// The main entry point when you already hold a [`Command`]. For a type that
/// derives [`clap::Parser`], use [`render_from`] instead.
pub fn render(cmd: &Command, opts: &Options) -> String {
    render::render(cmd, opts)
}

/// Render the CLI defined by `C` to a Markdown string using `opts`.
///
/// Obtains the [`Command`] via [`clap::CommandFactory`], so it works directly
/// with types deriving [`clap::Parser`].
pub fn render_from<C: CommandFactory>(opts: &Options) -> String {
    render::render(&C::command(), opts)
}
