//! Hardcoded values: page texts.

/// The page's section headings, in the order they appear.
pub const HEADING_VERSION: &str = "VERSION";
pub const HEADING_USAGE: &str = "HOW TO USE";
pub const HEADING_RELATED: &str = "RELATED";
pub const HEADING_SUBCOMMANDS: &str = "SUBCOMMANDS";
pub const HEADING_NOTES: &str = "NOTES";
pub const HEADING_FILES: &str = "FILES";
pub const HEADING_STEPS: &str = "STEPS";
pub const HEADING_USES: &str = "USES";

/// Words a recipe's step can start with that are not the command it uses:
/// wrappers and the shell's own words.
pub const RECIPE_SKIPPED_WORDS: &[&str] = &[
    "sudo", "doas", "env", "time", "nohup", "exec", "while", "until", "for", "do", "done", "if",
    "then", "else", "elif", "fi", "case", "esac", "in", "function", "select", "!", "[", "[[", "((",
    "local", "declare", "export", "readonly",
];

/// On a recipe's page: what it is, and the hints under its headings.
pub const IS_RECIPE: &str = "A recipe: a task done in steps, with the commands for each.";
pub const STEPS_HINT: &str = "In order; c copies a step's command. Read each before you run it.";
pub const USES_HINT: &str = "The commands it uses, each with its own page (/ to find one).";
pub const HEADING_LINKS: &str = "LINKS";
pub const HEADING_MANUAL: &str = "MANUAL";

/// What a name is, under its title, when it is not a program.
pub const IS_ALIAS: &str = "Your alias: typing it runs `{runs}`.";
pub const IS_BUILTIN: &str = "Built into your shell.";
pub const IS_KEYWORD: &str = "A word of your shell's language.";
pub const IS_FUNCTION: &str = "A shell function you defined.";
pub const IS_SUBCOMMAND: &str = "A subcommand of {program}.";

/// When a program has no page and `whatis` knows nothing of it.
pub const NO_SUMMARY: &str = "No summary yet for {name}.";
/// When a page has no examples.
pub const NO_EXAMPLES: &str =
    "No examples yet for {name}. Anyone can add them: see content/README.md in lore's repository.";
/// Under the examples, when one of them is marked dangerous.
pub const DANGER_EXAMPLE_NOTE: &str =
    "! marks an example that can destroy data or stop the system: check what it acts on first.";
/// Above the related commands and the subcommands.
pub const RELATED_HINT: &str = "Open one with / or `lore NAME`.";
pub const SUBCOMMANDS_HINT: &str =
    "Each has its own page: find it in the list, or run `lore {first}`.";

/// VERSION when there is no package to read it from.
pub const VERSION_ALIAS: &str =
    "An alias has no version of its own: it is the one of the command it runs.";
pub const VERSION_FUNCTION: &str = "A function you wrote has no version.";
pub const VERSION_SHELL: &str = "Part of your shell, so it has the shell's version (`bash --version`, `zsh --version`, `fish --version`).";
pub const VERSION_UNPACKAGED: &str = "Not from a package manager lore knows: installed by hand, by a language's tool (cargo, pip, npm…) or by a script. Its version: try `--version`.";
pub const VERSION_NOT_FOUND: &str =
    "Not installed on this machine, or not in a folder lore looks in.";

/// VERSION's fields, and the newest version's two cases.
pub const FIELD_PACKAGE: &str = "Package";
pub const FIELD_INSTALLED: &str = "Installed";
pub const FIELD_NEWEST: &str = "Newest";
pub const FIELD_UPDATE: &str = "Update";
pub const FIELD_LOCATION: &str = "Location";
pub const NEWEST_SAME: &str = "{newest}, up to date as of your last package list update";
pub const NEWEST_UPDATE: &str = "{newest}: an update is available";

/// How wide a field's label is, so the values line up.
pub const FIELD_LABEL_WIDTH: usize = 11;

/// A page's indents: of a section's text, of what an example does, of the
/// reasons there is no manual.
pub const PAGE_INDENT: &str = "  ";
pub const EXAMPLE_INDENT: &str = "      ";
pub const REASON_INDENT: &str = "    ";

/// Before an example, and before one that can destroy data.
pub const EXAMPLE_PROMPT: &str = "$ ";
pub const DANGER_MARK: &str = "! ";
