//! A tldr-pages page, turned into lore's page format (content/README.md),
//! for `lore --update-pages` when lore's own weekly pages cannot be had.
//! The same rules as scripts/content/common.py, which made the built-in
//! pages: tldr's format is
//!
//! ```text
//! # tar
//!
//! > Archiving utility.
//! > More information: <https://www.gnu.org/software/tar>.
//!
//! - [c]reate an archive:
//!
//! `tar cf {{path/to/target.tar}} {{path/to/file1}}`
//! ```

use crate::compile_config::{
    MAX_LINK_LENGTH, TLDR_MAX_EXAMPLES, TLDR_MORE_INFORMATION, TLDR_PAGE_LINK,
};
use crate::info::content::page_key;

/// `text`, tldr's page `name` (its file name, `git-commit`) from tldr's
/// `platform` folder, in lore's format. `english`: tldr's English labels
/// ("More information", "See also") are known; in a translation, any short
/// "label: `x`" line is taken as one, and "label: <https://…>" as the
/// link. None when it has no summary.
pub fn convert(name: &str, platform: &str, text: &str, english: bool) -> Option<String> {
    let mut title = name.to_owned();
    let mut summary: Vec<String> = Vec::new();
    let mut examples: Vec<(String, String)> = Vec::new();
    let mut more_info: Option<String> = None;
    let mut what: Option<String> = None;
    for raw in text.lines() {
        let line: String = raw.chars().filter(|c| !c.is_control()).collect();
        let line = line.trim_end();
        if let Some(rest) = line.strip_prefix("# ") {
            title = rest.trim().to_owned();
        } else if let Some(rest) = line.strip_prefix("> ") {
            let rest = rest.trim();
            let url = link_in(rest);
            let (is_link, refers) = if english {
                (
                    rest.starts_with("More information") || rest.starts_with("Homepage"),
                    rest.starts_with("See also"),
                )
            } else {
                // "Mais informações: <https://…>": a label, then the link;
                // a link inside a sentence is part of the summary
                let labelled = rest
                    .split("<https://")
                    .next()
                    .is_some_and(|before| before.trim_end().ends_with([':', '：']));
                (url.is_some() && labelled, labelled_reference(rest))
            };
            if is_link {
                more_info = more_info.or(url);
            } else if !refers {
                summary.push(rest.to_owned());
            }
        } else if let Some(rest) = line.strip_prefix("- ") {
            what = Some(describe(rest));
        } else if line.len() > 2
            && line.starts_with('`')
            && line.ends_with('`')
            && let Some(what) = what.take()
        {
            let typed = fill_placeholders(&line[1..line.len() - 1])
                .trim()
                .to_owned();
            if !typed.contains('`') && !typed.is_empty() {
                examples.push((typed, what));
            }
        }
    }
    if summary.is_empty() {
        return None;
    }
    // a subcommand keeps tldr's title (`git commit` in git-commit.md)
    if page_key(&title) != name {
        title = name.to_owned();
    }
    let mut page = format!("# {title}\n\n{}\n", summary.join(" "));
    if !examples.is_empty() {
        page.push_str("\n## Usage\n");
        for (typed, what) in examples.iter().take(TLDR_MAX_EXAMPLES) {
            page.push_str(&format!("- `{typed}` : {what}\n"));
        }
    }
    let tldr_link = TLDR_PAGE_LINK
        .replace("{platform}", platform)
        .replace("{name}", name);
    let links: Vec<(&str, String)> = more_info
        .map(|url| (TLDR_MORE_INFORMATION, url))
        .into_iter()
        .chain([("tldr page", tldr_link)])
        .filter(|(_, url)| url.starts_with("https://") && url.chars().count() <= MAX_LINK_LENGTH)
        .collect();
    if !links.is_empty() {
        page.push_str("\n## Links\n");
        for (label, url) in links {
            page.push_str(&format!("- {label}: {url}\n"));
        }
    }
    Some(page)
}

/// The first `<https://…>` in a line, without a final period.
fn link_in(line: &str) -> Option<String> {
    let start = line.find("<https://")? + 1;
    let end = start + line[start..].find('>')?;
    Some(line[start..end].trim_end_matches('.').to_owned())
}

/// A translation's "See also: `x`": a short label, a colon (or a
/// full-width one), then a backtick.
fn labelled_reference(line: &str) -> bool {
    let Some(colon) = line.find([':', '：']) else {
        return false;
    };
    let label = &line[..colon];
    let after = line[colon..].chars().skip(1).collect::<String>();
    label.chars().count() <= 40 && after.trim_start().starts_with('`')
}

/// tldr's `{{path/to/file}}` → `path/to/file`; `{{[-a|--all]}}` → `-a`.
pub fn fill_placeholders(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("{{") {
        let Some(end) = rest[start + 2..].find("}}") else {
            break;
        };
        out.push_str(&rest[..start]);
        let inner = &rest[start + 2..start + 2 + end];
        let choice = inner
            .strip_prefix('[')
            .and_then(|i| i.strip_suffix(']'))
            .filter(|i| i.contains('|'))
            .and_then(|i| i.split('|').next());
        out.push_str(choice.unwrap_or(inner));
        rest = &rest[start + 2 + end + 2..];
    }
    out.push_str(rest);
    out
}

/// An example's description as lore writes it: tldr's `[c]reate` marks
/// gone, no final colon or period, lower case first, unless it starts with
/// an acronym (`URL`) or a lone capital (`R` as in `chmod -R`).
pub fn describe(text: &str) -> String {
    // `[c]` around one letter: the letter
    let mut plain = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '['
            && chars.get(i + 2) == Some(&']')
            && chars.get(i + 1).is_some_and(|c| c.is_alphabetic())
        {
            plain.push(chars[i + 1]);
            i += 3;
        } else {
            plain.push(chars[i]);
            i += 1;
        }
    }
    let plain = fill_placeholders(plain.trim().trim_end_matches([':', '：', '.']).trim());
    let mut first_two = plain.chars().take(2);
    let acronym = first_two.next().is_some_and(char::is_uppercase)
        && !first_two.next().is_some_and(char::is_lowercase);
    if acronym {
        return plain;
    }
    let mut chars = plain.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().chain(chars).collect(),
        None => plain,
    }
}
