//! A command's manual page, as plain text lines. Runs the system's `man`
//! (no man-page parsing of our own), at the width it will be shown at, so the
//! page is laid out for the panel instead of being wrapped after the fact.

use std::process::{Command, Stdio};

/// Why a command's manual could not be shown. One general text for now: the
/// probable reasons, since `man` does not say which one it is.
pub const NO_PAGE: &str = "\
No manual page for this command.

Probable reasons:
  - it is part of the shell (a builtin or a keyword): try `help <name>` in
    bash, or the shell's own manual (`man bash`, `man zshbuiltins`)
  - it is one of your aliases: its page is the one of the command it runs
  - the program ships no manual page, or its documentation package is not
    installed (on Debian and Ubuntu some come as a separate `-doc` package)
  - manual pages were removed to save space (minimal and container images;
    on Ubuntu, `unminimize` brings them back)
  - `man` itself is not installed";

/// `name`'s manual laid out `width` columns wide, or None when there is no
/// page (or no `man`). Never fails otherwise: a page is a nice-to-have.
pub fn page(name: &str, width: u16) -> Option<Vec<String>> {
    let output = Command::new("man")
        .arg("--") // a name starting with `-` is a name, not an option
        .arg(name)
        .env("MANWIDTH", width.max(20).to_string()) // too narrow and man gives up
        .env("MANPAGER", "cat") // piped output skips the pager anyway; belt and braces
        .env("PAGER", "cat")
        .stdin(Stdio::null())
        .stderr(Stdio::null()) // "No manual entry for…" would scribble on the screen
        .output()
        .ok()?; // no `man` at all
    if !output.status.success() || output.stdout.is_empty() {
        return None;
    }
    Some(
        clean(&String::from_utf8_lossy(&output.stdout))
            .lines()
            .map(str::to_owned)
            .collect(),
    )
}

/// Plain text out of `man`'s output: drops the terminal formatting some
/// systems keep when piped. Bold is `c\bc`, underline `_\bc` (the character,
/// a backspace, the character again); colours are `ESC [ … m`.
pub fn clean(raw: &str) -> String {
    let mut text = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            // backspace: it overstrikes the character before it, which goes
            '\u{8}' => {
                text.pop();
            }
            // ESC [ … letter: a colour or style code, skipped whole
            '\u{1b}' if chars.peek() == Some(&'[') => {
                chars.next();
                for code in chars.by_ref() {
                    if code.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
            _ => text.push(c),
        }
    }
    text
}
