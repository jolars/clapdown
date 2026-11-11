//! A derive-based CLI fixture mirroring the real-world shape clapdown targets:
//! a root with a global option, flat subcommands, positionals (including a
//! variadic one), a value enum with per-variant docs, and a subcommand with a
//! distinct long about, plus before/after help with short and long variants.

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "demo", about = "A demo CLI for testing clapdown")]
#[command(
    before_help = "Short introduction.",
    before_long_help = "Welcome to the demo command reference.",
    after_help = "Short closing note.",
    after_long_help = "Choose a subcommand below for more details."
)]
pub struct Cli {
    /// Increase verbosity
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Parse a file
    #[command(
        before_help = "Inspect how a document is parsed.",
        after_help = "Parsing does not change the input file."
    )]
    Parse {
        /// Input file
        file: Option<String>,
        /// Only check, do not write
        #[arg(long)]
        check: bool,
    },

    /// Format files
    ///
    /// Formats every given path in place. Reads stdin when no paths are given.
    #[command(
        before_long_help = "Format documents consistently.",
        after_long_help = "Examples:\n\n```sh\ndemo format document.md\n```\n\n- Reads stdin when no paths are given.\n- Formats paths in place."
    )]
    Format {
        /// Paths to format
        path: Vec<String>,
        /// Output format
        #[arg(long, default_value = "pretty")]
        output: OutputFormat,
        /// Maximum line width
        #[arg(long, default_value = "80")]
        line_width: usize,
    },
}

#[derive(Copy, Clone, ValueEnum)]
pub enum OutputFormat {
    /// Annotated multi-line snippets
    Pretty,
    /// One finding per line
    Concise,
    /// Machine-readable JSON
    Json,
}
