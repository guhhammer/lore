//! Hardcoded values: lore explain.

/// `lore explain LINE`: the word that starts it.
pub const EXPLAIN_WORD: &str = "explain";

/// What the words of a command line mean when the man page does not say.
pub const EXPLAIN_NO_SUMMARY: &str = "no description found on this machine";
pub const EXPLAIN_NO_MANUAL: &str = "an option (no manual on this machine to say which)";
pub const EXPLAIN_NOT_IN_MANUAL: &str = "an option its manual does not list";
pub const EXPLAIN_VALUE_OF: &str = "the value given to {flag}";
pub const EXPLAIN_FILE_OF: &str = "the file for {op}";
pub const EXPLAIN_ARGUMENT: &str = "an argument: what it works on (a file, a name, a text)";
pub const EXPLAIN_SETTING: &str = "sets {name} for this command only";
pub const EXPLAIN_OPTIONS_END: &str = "the end of the options: what follows is taken as is";

/// The longest a flag's description gets, in characters (then `...`).
pub const EXPLAIN_MAX_CHARS: usize = 160;

/// Columns of the word column at most; a longer word gets its own line.
pub const EXPLAIN_WORD_WIDTH: usize = 18;

/// Programs that run another command after their own options: that
/// command is explained after them.
pub const EXPLAIN_WRAPPERS: &[&str] = &[
    "sudo", "doas", "env", "nice", "nohup", "time", "timeout", "xargs", "exec", "command", "watch",
    "strace", "ionice", "chroot", "flock", "stdbuf",
];

/// Programs whose first argument can be their flags without a dash
/// (`tar xzvf a.tgz`, `ps aux`).
pub const EXPLAIN_BUNDLED: &[&str] = &["tar", "ps", "ar"];

/// Between commands: what each joins.
pub const EXPLAIN_OPERATORS: &[(&str, &str)] = &[
    (
        "|",
        "sends the output of the command before into the next one",
    ),
    ("||", "runs the next command only if the one before failed"),
    ("&&", "runs the next command only if the one before worked"),
    (";", "then runs the next command"),
    ("&", "runs the command before in the background"),
];

/// Redirections: (as written, what it does, whether a file follows).
pub const EXPLAIN_REDIRECTS: &[(&str, &str, bool)] = &[
    (">", "writes the output to a file, replacing it", true),
    ("1>", "writes the output to a file, replacing it", true),
    (">>", "adds the output to the end of a file", true),
    ("<", "reads the input from a file", true),
    ("2>", "writes the error messages to a file", true),
    ("2>>", "adds the error messages to the end of a file", true),
    ("&>", "writes the output and the errors to a file", true),
    (
        "2>&1",
        "sends the error messages where the output goes",
        false,
    ),
    (">&2", "sends the output where the error messages go", false),
    (
        "1>&2",
        "sends the output where the error messages go",
        false,
    ),
];

/// Columns a man page is laid out at for reading its options: wide, so an
/// option's description is not wrapped onto many lines.
pub const EXPLAIN_MAN_WIDTH: u16 = 200;

/// `lore --print NAME` into a pipe or a file, where there is no terminal to
/// take the width from: columns to lay the page out at.
pub const PRINT_WIDTH: u16 = 80;
