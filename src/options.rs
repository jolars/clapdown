//! Configuration for Markdown rendering.

/// The Markdown flavor to target.
///
/// Different consumers accept slightly different Markdown extensions.
/// [`Flavor::Mdbook`] and [`Flavor::Pandoc`] are implemented; `CommonMark` is
/// planned. The enum is `#[non_exhaustive]` so new flavors can be added without
/// a breaking change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Flavor {
    /// [mdBook](https://rust-lang.github.io/mdBook/) Markdown. Renders
    /// definition lists (enabled by default in mdBook) for arguments.
    #[default]
    Mdbook,

    /// [Pandoc](https://pandoc.org/) Markdown. Emits the same definition lists
    /// and ATX headings as [`Flavor::Mdbook`], preceded by a YAML metadata
    /// block carrying the document `title` (see [`Options::metadata`]). With
    /// the title in the metadata, the root command's `h1` is omitted from the
    /// body to avoid duplicating it.
    Pandoc,
}

/// How a custom Pandoc metadata value is written to YAML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum MetadataStyle {
    /// Preserve newlines exactly, using a literal block for multiline text
    /// and quoting single-line text only when needed.
    #[default]
    Auto,

    /// Use a folded block (`>`). YAML folds ordinary line breaks into spaces.
    /// A nonempty value retains at least one final newline, allowing Pandoc
    /// to interpret ordinary text as a paragraph.
    Folded,

    /// Use a literal block (`|`), preserving line breaks and indentation.
    /// A nonempty value retains at least one final newline. Pandoc still
    /// interprets the value as Markdown; use code fences inside the value
    /// for a verbatim code block.
    Literal,
}

/// Options controlling how a [`clap::Command`] is rendered to Markdown.
///
/// Construct with [`Options::new`] (or [`Options::default`]) and configure with
/// the chained builder methods, then pass it to the crate's `render` or
/// `render_from` function.
///
/// ```
/// # use clap::Command;
/// let cmd = Command::new("demo").about("A demo");
/// let md = clapdown::render(
///     &cmd,
///     &clapdown::Options::new().base_heading_level(2).footer(false),
/// );
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
    pub(crate) metadata: bool,
    pub(crate) metadata_fields: Vec<(String, String, MetadataStyle)>,
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
            metadata: true,
            metadata_fields: Vec::new(),
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

    /// Emit a Pandoc YAML metadata block (default on, [`Flavor::Pandoc`] only).
    ///
    /// The block always carries a `title` (the [`Options::title`] override if
    /// set, otherwise the root command name) plus any custom
    /// [`Options::metadata_field`]s. When the block is emitted, its `title`
    /// stands in for the document's top-level heading, so the root command's
    /// `h1` is omitted from the body. Has no effect for other flavors.
    pub fn metadata(mut self, on: bool) -> Self {
        self.metadata = on;
        self
    }

    /// Add a custom field to the Pandoc YAML metadata block.
    ///
    /// Fields are emitted in insertion order after `title`; a `title` key is
    /// ignored here (use [`Options::title`] instead). Only affects
    /// [`Flavor::Pandoc`] with [`Options::metadata`] enabled. Can be called
    /// repeatedly to add several fields.
    ///
    /// Values retain their newlines, including trailing newlines. Use
    /// [`Options::metadata_field_with_style`] to request a folded or literal
    /// block explicitly.
    ///
    /// ```
    /// # use clap::Command;
    /// # use clapdown::{Flavor, Options, render};
    /// let cmd = Command::new("demo").about("A demo");
    /// let md = render(
    ///     &cmd,
    ///     &Options::new()
    ///         .flavor(Flavor::Pandoc)
    ///         .metadata_field("author", "Jane Doe"),
    /// );
    /// assert!(md.contains("author: Jane Doe"));
    /// ```
    pub fn metadata_field(self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata_field_with_style(key, value, MetadataStyle::Auto)
    }

    /// Add a custom Pandoc metadata field with an explicit YAML style.
    ///
    /// Has the same ordering, flavor, and `title` rules as
    /// [`Options::metadata_field`]. Folded and literal blocks retain at least
    /// one final newline for nonempty values. Values containing control
    /// characters that cannot appear in a block use YAML escapes instead.
    ///
    /// ```
    /// # use clap::Command;
    /// use clapdown::{Flavor, MetadataStyle, Options, render};
    /// let options = Options::new()
    ///     .flavor(Flavor::Pandoc)
    ///     .metadata_field_with_style("description", "A CLI reference.", MetadataStyle::Folded);
    /// let md = render(&Command::new("demo"), &options);
    /// assert!(md.contains("description: >\n  A CLI reference.\n"));
    /// ```
    pub fn metadata_field_with_style(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
        style: MetadataStyle,
    ) -> Self {
        self.metadata_fields.push((key.into(), value.into(), style));
        self
    }
}
