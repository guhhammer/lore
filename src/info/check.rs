//! The check for pages lore did not write: a page another program installs
//! (`/usr/share/lore/content/NAME.md`, its translations in `i18n/<lang>/`),
//! or one you added. lore never runs what a page says, but a page is shown
//! in a terminal and read by people, so it must be plain text that cannot
//! drive the terminal, hide or reorder what it says, or hand someone a
//! command that runs a download.
//!
//! Errors keep the page from being shown (and `lore check` exits with 1);
//! warnings are shown by `lore check` only. Pages built into lore were
//! checked when they were written and are not checked again here.

use crate::configs::{
    CHECK_HTML, CHECK_INVISIBLE, CHECK_SECTIONS, MAX_LINK_LENGTH, PAGE_MAX_BYTES,
};
use crate::info::content::page_key;
use crate::info::danger;

/// One thing the check found, on a line (1 = the first; 0 = the file).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub line: usize,
    pub error: bool,
    pub text: String,
}

/// Whether nothing found is an error: the page may be shown.
pub fn passes(findings: &[Finding]) -> bool {
    findings.iter().all(|f| !f.error)
}

/// Checks the page file `name.md` (its file name without `.md`), given as
/// bytes, as read from the disk.
pub fn check(name: &str, bytes: &[u8]) -> Vec<Finding> {
    let mut found = Vec::new();
    let error = |line: usize, text: String| Finding {
        line,
        error: true,
        text,
    };
    let warning = |line: usize, text: String| Finding {
        line,
        error: false,
        text,
    };
    if bytes.len() > PAGE_MAX_BYTES {
        found.push(error(
            0,
            format!("larger than {PAGE_MAX_BYTES} bytes: a page is a short text"),
        ));
        return found;
    }
    let Ok(text) = std::str::from_utf8(bytes) else {
        found.push(error(0, "not UTF-8 text".to_owned()));
        return found;
    };

    // the title: the page's own name, so a file cannot pose as another page
    match text.lines().position(|l| !l.trim().is_empty()) {
        Some(i) => {
            let first = text.lines().nth(i).unwrap_or_default();
            match first.strip_prefix("# ") {
                Some(title) if page_key(title.trim()) == name => {}
                Some(title) => found.push(error(
                    i + 1,
                    format!(
                        "the title \"{}\" is another page's name, not {name}'s",
                        title.trim()
                    ),
                )),
                None => found.push(error(i + 1, "the first line is not `# NAME`".to_owned())),
            }
        }
        None => found.push(error(0, "the page is empty".to_owned())),
    }

    let mut section = String::new();
    for (i, line) in text.lines().enumerate() {
        let at = i + 1;
        for c in line.chars() {
            if c.is_control() && c != '\t' && c != '\r' {
                found.push(error(
                    at,
                    format!(
                        "a control character (U+{:04X}): a terminal could obey it",
                        c as u32
                    ),
                ));
                break;
            }
            if CHECK_INVISIBLE
                .iter()
                .any(|(from, to)| (*from..=*to).contains(&c))
            {
                found.push(error(
                    at,
                    format!(
                        "an invisible character (U+{:04X}) that can hide or reorder text",
                        c as u32
                    ),
                ));
                break;
            }
        }
        let lower = line.to_lowercase();
        if let Some(tag) = CHECK_HTML.iter().find(|tag| has_code(&lower, tag)) {
            found.push(error(
                at,
                format!("`{tag}`: HTML or script; a page is plain text"),
            ));
        }
        if let Some(heading) = line.strip_prefix("## ") {
            section = heading.trim().to_lowercase();
            if !CHECK_SECTIONS.contains(&section.as_str()) {
                found.push(warning(
                    at,
                    format!("the section \"{}\" is not shown", heading.trim()),
                ));
            }
            continue;
        }
        match section.as_str() {
            "usage" | "steps" | "tasks" => {
                let Some(typed) = example(line) else {
                    continue;
                };
                if runs_a_download(typed) {
                    found.push(error(
                        at,
                        "the example runs what it downloads (`curl … | sh`): no page may ask for that"
                            .to_owned(),
                    ));
                } else if danger::example(typed) {
                    found.push(warning(
                        at,
                        "a risky example: lore marks it with !".to_owned(),
                    ));
                }
            }
            "links" => {
                if let Some(url) = line.trim().split_once(": ").map(|(_, url)| url.trim()) {
                    for problem in link_problems(url) {
                        found.push(warning(at, problem));
                    }
                }
            }
            _ => {}
        }
    }
    found
}

/// Whether `line` has `pattern` as code: `<script` as a tag (`<script>`,
/// `<script src=…>`), not as the start of a file name in a shell
/// redirection (`scheme <script.scm`).
fn has_code(line: &str, pattern: &str) -> bool {
    let ends_in_a_name = pattern.ends_with(|c: char| c.is_ascii_alphabetic());
    line.match_indices(pattern).any(|(at, _)| {
        let next = line[at + pattern.len()..].chars().next();
        !ends_in_a_name || !next.is_some_and(|c| c.is_alphanumeric() || "._-".contains(c))
    })
}

/// The command of a `- `cmd` : what` line.
fn example(line: &str) -> Option<&str> {
    let rest = line.trim().strip_prefix("- `")?;
    rest.split_once('`').map(|(typed, _)| typed)
}

/// `curl URL | sh`, `bash <(wget …)`, `eval "$(curl …)"`: a command that
/// runs whatever a server sends.
pub fn runs_a_download(typed: &str) -> bool {
    let compact: String = typed
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let fetches = compact.contains("curl") || compact.contains("wget");
    if !fetches {
        return false;
    }
    let piped = [
        "|sh",
        "|bash",
        "|zsh",
        "|sudosh",
        "|sudobash",
        "|sudo-ebash",
        "|python",
    ]
    .iter()
    .any(|p| compact.contains(p));
    let run_in_place = [
        "sh-c", "bash-c", "zsh-c", "eval", "source<(", "bash<(", "sh<(", ".<(",
    ]
    .iter()
    .any(|p| compact.contains(p));
    piped || run_in_place
}

/// What is odd about a link; lore shows only https links of a safe length.
pub fn link_problems(url: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let Some(rest) = url.strip_prefix("https://") else {
        problems.push("not an https link: lore does not show it".to_owned());
        return problems;
    };
    if url.chars().count() > MAX_LINK_LENGTH {
        problems.push(format!(
            "longer than {MAX_LINK_LENGTH} characters: a narrow screen cuts it"
        ));
    }
    let authority = rest.split(['/', '?', '#']).next().unwrap_or_default();
    if authority.contains('@') {
        problems
            .push("a user or password in the link: it can hide where it really goes".to_owned());
    }
    let host = authority.rsplit('@').next().unwrap_or_default();
    let host = host.split(':').next().unwrap_or_default();
    if host.split('.').any(|label| label.starts_with("xn--")) {
        problems.push("a punycode name (xn--): it can look like another site".to_owned());
    }
    if host.is_empty()
        || host.chars().all(|c| c.is_ascii_digit() || c == '.')
        || authority.starts_with('[')
    {
        problems.push("an address, not a name: say which site it is".to_owned());
    }
    problems
}
