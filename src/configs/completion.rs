//! Hardcoded values: shell completion (src/interface/complete.rs, completions/).

/// `lore --complete WORDS` (the scripts ask it) and `lore --completions
/// SHELL` (prints a script).
pub const COMPLETE_FLAG: &str = "--complete";
pub const COMPLETIONS_FLAG: &str = "--completions";

/// What can start the command line, offered after `lore -` (and `explain`
/// after `lore e`).
pub const COMPLETE_OPTIONS: &[(&str, &str)] = &[
    ("--print", "a page as plain text, for pipes and scripts"),
    ("-p", "a page as plain text, for pipes and scripts"),
    ("--help", "how to use lore"),
    ("-h", "how to use lore"),
    ("--version", "lore's version"),
    ("-V", "lore's version"),
    (
        "--completions",
        "print the completion script for bash, zsh or fish",
    ),
    ("explain", "each word of a command line, from the man pages"),
    (
        "--update-pages",
        "download newer pages (the only time lore asks for them)",
    ),
];

/// Candidates get a description (the page's first sentence) only when
/// there are at most this many, and it is cut to this many columns.
pub const COMPLETE_DESCRIBE_MAX: usize = 300;
pub const COMPLETE_DESCRIBE_WIDTH: usize = 60;

/// The completion scripts, by shell, for `lore --completions SHELL`.
pub const COMPLETION_SCRIPTS: &[(&str, &str)] = &[
    ("bash", include_str!("../../completions/lore.bash")),
    ("zsh", include_str!("../../completions/_lore")),
    ("fish", include_str!("../../completions/lore.fish")),
];
pub const ERROR_COMPLETIONS_SHELL: &str = "--completions needs a shell: bash, zsh or fish";
