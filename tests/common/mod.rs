//! A derive-based CLI fixture mirroring the real-world shape clapdown targets:
//! a root with a global option, flat subcommands, positionals (including a
//! variadic one), a value enum with per-variant docs, and a subcommand with a
//! distinct long about.

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "demo", about = "A demo CLI for testing clapdown")]
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
