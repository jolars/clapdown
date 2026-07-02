mod common;

use clap::{Command, CommandFactory};
use clapdown::Options;
use common::Cli;

/// Extract the level of every ATX heading (`#`..`######`) in order.
fn heading_levels(md: &str) -> Vec<usize> {
    md.lines()
        .filter_map(|line| {
            let hashes = line.chars().take_while(|c| *c == '#').count();
            if (1..=6).contains(&hashes) && line.chars().nth(hashes) == Some(' ') {
                Some(hashes)
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn headings_never_skip_a_level() {
    let md = Options::new().render(&Cli::command());
    let levels = heading_levels(&md);

    assert!(!levels.is_empty(), "expected at least one heading");
    assert_eq!(levels[0], 1, "root heading should be at the base level");
    for pair in levels.windows(2) {
        assert!(
            pair[1] <= pair[0] + 1,
            "heading outline skipped a level: {} -> {}",
            pair[0],
            pair[1]
        );
    }
}

#[test]
fn base_heading_level_shifts_every_heading() {
    let cmd = Cli::command();
    let base1 = heading_levels(&Options::new().base_heading_level(1).render(&cmd));
    let base3 = heading_levels(&Options::new().base_heading_level(3).render(&cmd));

    assert_eq!(base1.len(), base3.len());
    for (a, b) in base1.iter().zip(&base3) {
        assert_eq!(*b, *a + 2, "base_heading_level should offset every heading");
    }
}

#[test]
fn subcommands_nest_one_level_below_parent() {
    let md = Options::new().render(&Cli::command());
    assert!(md.contains("# `demo`\n"), "root command at h1");
    assert!(md.contains("## `demo parse`\n"), "subcommand at h2");
    assert!(
        md.contains("### Arguments\n"),
        "subcommand section at h3, one below the subcommand"
    );
}

#[test]
fn long_about_is_preferred_over_short_about() {
    let md = Options::new().render(&Cli::command());
    assert!(
        md.contains("Formats every given path in place."),
        "the long about of `format` should be rendered"
    );
}

#[test]
fn option_renders_as_definition_list_with_defaults_and_values() {
    let md = Options::new().render(&Cli::command());

    assert!(
        md.contains("`--output <OUTPUT>`\n:   Output format"),
        "term line then definition marker"
    );
    assert!(md.contains("Default value: `pretty`"));
    assert!(md.contains("Possible values:"));
    assert!(md.contains("- `pretty`: Annotated multi-line snippets"));
    assert!(md.contains("- `json`: Machine-readable JSON"));
}

#[test]
fn variadic_positional_gets_ellipsis() {
    let md = Options::new().render(&Cli::command());
    assert!(
        md.contains("`<PATH>...`"),
        "variadic positional shows `...`"
    );
}

#[test]
fn boolean_flag_has_no_value_placeholder() {
    let md = Options::new().render(&Cli::command());
    assert!(md.contains("`-v`, `--verbose`"));
    assert!(!md.contains("--verbose <"));
}

#[test]
fn title_overrides_root_heading() {
    let md = Options::new()
        .title("Command-Line Reference")
        .render(&Cli::command());
    assert!(md.starts_with("# Command-Line Reference\n"));
    assert!(!md.contains("# `demo`\n"));
}

/// Build a chain of nested subcommands from a list of static names.
fn nest(names: &[&'static str]) -> Command {
    let (first, rest) = names.split_first().expect("at least one name");
    let mut cmd = Command::new(*first).about("nested");
    if !rest.is_empty() {
        cmd = cmd.subcommand(nest(rest));
    }
    cmd
}

#[test]
fn deep_nesting_falls_back_to_bold_beyond_h6() {
    // base 1 + depth 6 = level 7, which cannot be an ATX heading.
    let md = Options::new().render(&nest(&["c0", "c1", "c2", "c3", "c4", "c5", "c6"]));

    assert!(
        !md.lines().any(|l| l.starts_with("####### ")),
        "there is no h7"
    );
    assert!(
        md.contains("**`c0 c1 c2 c3 c4 c5 c6`**"),
        "the depth-6 command falls back to a bold label"
    );
}

#[test]
fn pandoc_emits_yaml_metadata_block_with_title() {
    use clapdown::Flavor;
    let md = Options::new()
        .flavor(Flavor::Pandoc)
        .render(&Cli::command());
    assert!(
        md.starts_with("---\ntitle: demo\n"),
        "leading metadata block"
    );
    assert!(md.contains("\n---\n\n"), "metadata block is closed");
}

#[test]
fn pandoc_omits_root_h1_when_metadata_present() {
    use clapdown::Flavor;
    let md = Options::new()
        .flavor(Flavor::Pandoc)
        .render(&Cli::command());
    assert!(
        !md.lines().any(|l| l == "# `demo`"),
        "root h1 replaced by the metadata title"
    );
    assert!(md.contains("## `demo parse`"), "subcommands still render");
}

#[test]
fn pandoc_title_override_goes_into_metadata() {
    use clapdown::Flavor;
    let md = Options::new()
        .flavor(Flavor::Pandoc)
        .title("My Tool")
        .render(&Cli::command());
    assert!(md.starts_with("---\ntitle: My Tool\n"));
}

#[test]
fn pandoc_metadata_field_injects_custom_field() {
    use clapdown::Flavor;
    let md = Options::new()
        .flavor(Flavor::Pandoc)
        .metadata_field("author", "Jane Doe")
        .render(&Cli::command());
    assert!(md.contains("\nauthor: Jane Doe\n"));
}

#[test]
fn pandoc_without_metadata_matches_mdbook() {
    use clapdown::Flavor;
    let cmd = Cli::command();
    let mdbook = Options::new().flavor(Flavor::Mdbook).render(&cmd);
    let pandoc = Options::new()
        .flavor(Flavor::Pandoc)
        .metadata(false)
        .render(&cmd);
    assert_eq!(
        pandoc, mdbook,
        "with the metadata block off, Pandoc output equals mdBook"
    );
}

#[test]
fn full_document_snapshot() {
    let md = Options::new().render(&Cli::command());
    insta::assert_snapshot!(md);
}

#[test]
fn pandoc_full_document_snapshot() {
    use clapdown::Flavor;
    let md = Options::new()
        .flavor(Flavor::Pandoc)
        .metadata_field("author", "Jane Doe")
        .render(&Cli::command());
    insta::assert_snapshot!(md);
}
