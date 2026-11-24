//! Tests for `lore explain` (src/info/explain.rs).

use lore_tui::info::explain::{Lookup, explain, options_in_man, tokens};

fn lines(text: &str) -> Vec<String> {
    text.lines().map(str::to_owned).collect()
}

const TAR_MAN: &str = "\
NAME
       tar - an archiving utility
OPTIONS
       -x, --extract, --get
              Extract  files  from an archive.  Arguments are optional.

       -z, --gzip
              Filter the archive through gzip(1).

       -v, --verbose
              Verbosely list files processed.

       -f, --file=ARCHIVE
              Use archive file or device ARCHIVE.
";

const LS_MAN: &str = "\
DESCRIPTION
       -a, --all
              do not ignore entries starting with .

       -l     use a long listing format
";

/// Explains `line` with tar's and ls's manuals, and summaries for
/// tar, ls and sudo (sudo has no manual here).
fn run(line: &str) -> Vec<(String, String)> {
    let manual = |name: &str| match name {
        "tar" => Some(lines(TAR_MAN)),
        "ls" => Some(lines(LS_MAN)),
        _ => None,
    };
    let summary = |name: &str| match name {
        "tar" => Some("Packs files. And more.".to_owned()),
        "ls" => Some("Lists files.".to_owned()),
        _ => None,
    };
    explain(
        line,
        &Lookup {
            manual: &manual,
            summary: &summary,
        },
    )
}

fn words(rows: &[(String, String)]) -> Vec<&str> {
    rows.iter().map(|(w, _)| w.as_str()).collect()
}

#[test]
fn options_are_read_with_every_spelling_and_their_first_sentence() {
    let options = options_in_man(&lines(TAR_MAN));
    assert_eq!(options[0].names, ["-x", "--extract", "--get"]);
    assert_eq!(options[0].text, "Extract files from an archive.");
    assert!(!options[0].takes_value);
    assert!(options[3].takes_value, "--file=ARCHIVE takes a value");
}

#[test]
fn an_option_with_its_text_on_the_same_line_is_read_too() {
    let options = options_in_man(&lines(LS_MAN));
    assert_eq!(options[1].names, ["-l"]);
    assert_eq!(options[1].text, "use a long listing format");
}

#[test]
fn joined_short_flags_are_split_and_the_last_takes_its_value() {
    let rows = run("tar -xzvf a.tgz");
    assert_eq!(words(&rows), ["tar", "-x", "-z", "-v", "-f", "a.tgz"]);
    assert_eq!(rows[0].1, "Packs files.", "a summary's first sentence");
    assert_eq!(rows[1].1, "Extract files from an archive.");
    assert_eq!(rows[5].1, "the value given to -f");
}

#[test]
fn tars_old_flags_without_a_dash_are_read_as_flags() {
    assert_eq!(
        words(&run("tar xzf a.tgz")),
        ["tar", "-x", "-z", "-f", "a.tgz"]
    );
}

#[test]
fn a_long_option_with_its_value_after_an_equals_sign_is_split() {
    let rows = run("tar --file=a.tar -x");
    assert_eq!(words(&rows), ["tar", "--file", "a.tar", "-x"]);
}

#[test]
fn every_command_of_a_pipe_is_explained_with_the_pipe_between() {
    let rows = run("ls -l | tar -x");
    assert_eq!(words(&rows), ["ls", "-l", "|", "tar", "-x"]);
}

#[test]
fn sudo_is_explained_then_the_command_it_runs() {
    let rows = run("sudo ls -a");
    assert_eq!(words(&rows), ["sudo", "ls", "-a"]);
    assert_eq!(rows[2].1, "do not ignore entries starting with .");
}

#[test]
fn settings_redirections_and_unknown_flags_say_what_they_are() {
    let rows = run("LANG=C ls -Z > out 2>&1");
    assert_eq!(words(&rows), ["LANG=C", "ls", "-Z", ">", "out", "2>&1"]);
    assert!(rows[0].1.contains("LANG"));
    assert!(rows[2].1.contains("does not list"));
}

#[test]
fn a_flag_of_a_program_with_no_manual_says_there_is_no_manual() {
    let rows = run("mystery -q");
    assert!(rows[1].1.contains("no manual"));
}

#[test]
fn words_are_split_like_a_shell_splits_them() {
    assert_eq!(
        tokens("git commit -m 'fix it'"),
        ["git", "commit", "-m", "fix it"]
    );
    assert_eq!(tokens(r#"echo "a b" c\ d"#), ["echo", "a b", "c d"]);
    assert_eq!(tokens("a|b&&c;d"), ["a", "|", "b", "&&", "c", ";", "d"]);
    assert_eq!(tokens("x >out 2>&1"), ["x", ">", "out", "2>&1"]);
}
