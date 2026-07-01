//! Configuration for Markdown rendering.

/// The Markdown flavor to target.
///
/// Different consumers accept slightly different Markdown extensions. For now
/// only [`Flavor::Mdbook`] is implemented; `CommonMark` and `Pandoc` are
/// planned. The enum is `#[non_exhaustive]` so new flavors can be added without
/// a breaking change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Flavor {
    /// [mdBook](https://rust-lang.github.io/mdBook/) Markdown. Renders
    /// definition lists (enabled by default in mdBook) for arguments.
    #[default]
    Mdbook,
}

/// Options controlling how a [`clap::Command`] is rendered to Markdown.
///
/// Construct with [`Options::new`] (or [`Options::default`]) and configure with
/// the chained builder methods, then call [`Options::render`].
///
/// ```
/// # use clap::Command;
/// let cmd = Command::new("demo").about("A demo");
/// let md = clapdown::Options::new()
///     .base_heading_level(2)
///     .footer(false)
///     .render(&cmd);
/// assert!(md.starts_with("## `demo`"));
/// ```
#[derive(Debug, Clone)]
pub struct Options {
    pub(crate) flavor: Flavor,
    pub(crate) base_heading_level: u8,
    pub(crate) title: Option<String>,
    pub(crate) table_of_contents: bool,
    pub(crate) footer: bool,
    pub(crate) aliases: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            flavor: Flavor::Mdbook,
            base_heading_level: 1,
            title: None,
            table_of_contents: false,
            footer: false,
            aliases: true,
        }
    }
}

impl Options {
    /// Create a new [`Options`] with default settings.
    ///
    /// Defaults: [`Flavor::Mdbook`], base heading level 1, no title override, no
    /// table of contents, no footer, aliases shown.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the output [`Flavor`].
    pub fn flavor(mut self, flavor: Flavor) -> Self {
        self.flavor = flavor;
        self
    }

    /// Set the heading level of the root command (1 = `#`, 2 = `##`, ...).
    ///
    /// Subcommands descend one level per nesting depth, and each command's
    /// `Arguments`/`Options` sections sit one level below the command. Levels
    /// are clamped to a maximum of 6; anything deeper falls back to a bold
    /// label.
    pub fn base_heading_level(mut self, level: u8) -> Self {
        self.base_heading_level = level;
        self
    }

    /// Override the text of the root command's heading.
    ///
    /// By default the root heading is the command's own name. This replaces it
    /// (for example with `"Command-Line Reference"`).
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Emit a table of contents linking to each command (default off).
    pub fn table_of_contents(mut self, on: bool) -> Self {
        self.table_of_contents = on;
        self
    }

    /// Emit an attribution footer (default off).
    pub fn footer(mut self, on: bool) -> Self {
        self.footer = on;
        self
    }

    /// Show command and argument aliases (default on).
    pub fn aliases(mut self, on: bool) -> Self {
        self.aliases = on;
        self
    }

    /// Render `cmd` to a Markdown string using these options.
    pub fn render(&self, cmd: &clap::Command) -> String {
        crate::render::render(cmd, self)
    }
}
