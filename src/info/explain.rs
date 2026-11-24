//! `lore explain "tar -xzvf a.tgz"`: each word of a command line and what
//! it means, offline. A program's meaning is its lore page's summary (or its
//! `whatis` line); a flag's is the first sentence its man page gives it under
//! that flag. Pipes and lists (`|`, `&&`, `;`) are split and each command
//! explained; `sudo`, `env` and the like are explained, then the command
//! they run.

use crate::compile_config::{
    EXPLAIN_ARGUMENT, EXPLAIN_BUNDLED, EXPLAIN_FILE_OF, EXPLAIN_MAX_CHARS, EXPLAIN_NO_MANUAL,
    EXPLAIN_NO_SUMMARY, EXPLAIN_NOT_IN_MANUAL, EXPLAIN_OPERATORS, EXPLAIN_OPTIONS_END,
    EXPLAIN_REDIRECTS, EXPLAIN_SETTING, EXPLAIN_VALUE_OF, EXPLAIN_WRAPPERS,
};
use crate::info::content::page_key;

/// One option as a man page lists it: `-f, --file=ARCHIVE` and its text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManOption {
    /// Every spelling: `-f`, `--file`.
    pub names: Vec<String>,
    /// Takes a value: `--file=ARCHIVE`, `-name pattern`.
    pub takes_value: bool,
    /// The first sentence of its description.
    pub text: String,
}

/// What lore needs to know about a program: its manual's lines (by page
/// name, `git-commit`) and a one-line summary. Separate so tests can give
/// their own.
pub struct Lookup<'a> {
    pub manual: &'a dyn Fn(&str) -> Option<Vec<String>>,
    pub summary: &'a dyn Fn(&str) -> Option<String>,
}

/// The command line's words, each with what it means, in order.
pub fn explain(line: &str, lookup: &Lookup) -> Vec<(String, String)> {
    let mut rows = Vec::new();
    let mut command: Vec<String> = Vec::new();
    for token in tokens(line) {
        if let Some(meaning) = operator(&token) {
            explain_command(&command, lookup, &mut rows);
            command.clear();
            rows.push((token, meaning.to_owned()));
        } else {
            command.push(token);
        }
    }
    explain_command(&command, lookup, &mut rows);
    rows
}

/// What a `|`, `&&`, `;`… between commands does; None for any other word.
fn operator(token: &str) -> Option<&'static str> {
    EXPLAIN_OPERATORS
        .iter()
        .find(|(op, _)| *op == token)
        .map(|(_, meaning)| *meaning)
}

/// One command's words (no operators) into `rows`.
fn explain_command(words: &[String], lookup: &Lookup, rows: &mut Vec<(String, String)>) {
    let mut at = 0;
    // `LANG=C sort`: settings before the program
    while let Some(word) = words.get(at).filter(|w| is_setting(w)) {
        let name = word.split('=').next().unwrap_or_default();
        rows.push((word.clone(), EXPLAIN_SETTING.replace("{name}", name)));
        at += 1;
    }
    let Some(program) = words.get(at) else {
        return;
    };
    let program = program.rsplit('/').next().unwrap_or(program).to_owned(); // /usr/bin/ls → ls
    at += 1;
    // `git commit`: a subcommand with a manual of its own
    let mut name = program.clone();
    let mut shown = words[at - 1].clone();
    if let Some(sub) = words.get(at).filter(|w| !w.starts_with('-')) {
        let joined = format!("{program} {sub}");
        if (lookup.manual)(&page_key(&joined)).is_some() {
            shown = format!("{shown} {sub}");
            name = joined;
            at += 1;
        }
    }
    let manual = (lookup.manual)(&page_key(&name));
    let options = manual.as_deref().map(options_in_man).unwrap_or_default();
    let summary = (lookup.summary)(&name)
        .map(|text| first_sentence(&text))
        .unwrap_or_else(|| EXPLAIN_NO_SUMMARY.to_owned());
    rows.push((shown, summary));

    let flag_text = |found: Option<&ManOption>| match (found, &manual) {
        (Some(option), _) => option.text.clone(),
        (None, Some(_)) => EXPLAIN_NOT_IN_MANUAL.to_owned(),
        (None, None) => EXPLAIN_NO_MANUAL.to_owned(),
    };
    let wrapper = EXPLAIN_WRAPPERS.contains(&program.as_str());
    let mut options_ended = false;
    let mut first_operand = true;
    while at < words.len() {
        let word = &words[at];
        at += 1;
        if let Some((_, meaning, to_file)) = EXPLAIN_REDIRECTS.iter().find(|(r, ..)| word == r) {
            rows.push((word.clone(), (*meaning).to_owned()));
            if let Some(target) = words.get(at).filter(|_| *to_file) {
                rows.push((target.clone(), EXPLAIN_FILE_OF.replace("{op}", word)));
                at += 1;
            }
            continue;
        }
        if word == "--" && !options_ended {
            rows.push((word.clone(), EXPLAIN_OPTIONS_END.to_owned()));
            options_ended = true;
            continue;
        }
        let is_flag = !options_ended && word.len() > 1 && word.starts_with('-');
        if is_flag {
            let (flag, inline_value) = match word.split_once('=') {
                Some((flag, value)) if word.starts_with("--") => (flag, Some(value)),
                _ => (word.as_str(), None),
            };
            if let Some(option) = find(&options, flag) {
                rows.push((flag.to_owned(), option.text.clone()));
                if let Some(value) = inline_value {
                    rows.push((value.to_owned(), EXPLAIN_VALUE_OF.replace("{flag}", flag)));
                } else if option.takes_value
                    && let Some(value) = words.get(at)
                {
                    rows.push((value.clone(), EXPLAIN_VALUE_OF.replace("{flag}", flag)));
                    at += 1;
                }
            } else if flag.starts_with("--") || flag.len() == 2 {
                rows.push((flag.to_owned(), flag_text(None)));
            } else {
                // `-xzvf`: -x -z -v -f; the one that takes a value ends it
                at = cluster(&flag[1..], &options, words, at, rows, &flag_text);
            }
            continue;
        }
        if wrapper && first_operand {
            // `sudo apt install x`: the rest is a command of its own
            explain_command(&words[at - 1..], lookup, rows);
            return;
        }
        if first_operand
            && EXPLAIN_BUNDLED.contains(&program.as_str())
            && word.chars().all(|c| c.is_ascii_alphabetic())
            && word
                .chars()
                .all(|c| find(&options, &format!("-{c}")).is_some())
        {
            // `tar xzvf`: the old way, the letters without a dash
            first_operand = false;
            at = cluster(word, &options, words, at, rows, &flag_text);
            continue;
        }
        first_operand = false;
        rows.push((word.clone(), EXPLAIN_ARGUMENT.to_owned()));
    }
}

/// The short flags in `letters` (`xzvf`) into `rows`; the first that takes a
/// value takes the rest of the letters, or else the next word. Returns where
/// the words go on.
fn cluster(
    letters: &str,
    options: &[ManOption],
    words: &[String],
    mut at: usize,
    rows: &mut Vec<(String, String)>,
    flag_text: &dyn Fn(Option<&ManOption>) -> String,
) -> usize {
    for (i, letter) in letters.char_indices() {
        let flag = format!("-{letter}");
        let found = find(options, &flag);
        rows.push((flag.clone(), flag_text(found)));
        if found.is_some_and(|o| o.takes_value) {
            let rest = &letters[i + letter.len_utf8()..];
            let value = if rest.is_empty() {
                let next = words.get(at).cloned();
                at += usize::from(next.is_some());
                next
            } else {
                Some(rest.to_owned()) // `-fa.tgz`
            };
            if let Some(value) = value {
                rows.push((value, EXPLAIN_VALUE_OF.replace("{flag}", &flag)));
            }
            break;
        }
    }
    at
}

fn find<'a>(options: &'a [ManOption], flag: &str) -> Option<&'a ManOption> {
    options.iter().find(|o| o.names.iter().any(|n| n == flag))
}

/// `LANG=C`: a setting for the command, not the command.
fn is_setting(word: &str) -> bool {
    word.split_once('=').is_some_and(|(name, _)| {
        !name.is_empty()
            && !name.starts_with(|c: char| c.is_ascii_digit())
            && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    })
}

/// The words of a command line, as a shell would split them: quotes keep
/// spaces ('a b', "a b"), a backslash keeps the next character, and `|`,
/// `||`, `&&`, `;` and `&` are words of their own even without spaces.
pub fn tokens(line: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut word = String::new();
    let mut quoted = false; // the word had quotes: '' is a word too
    let mut chars = line.chars().peekable();
    let end = |word: &mut String, quoted: &mut bool, found: &mut Vec<String>| {
        if !word.is_empty() || *quoted {
            found.push(std::mem::take(word));
        }
        *quoted = false;
    };
    while let Some(c) = chars.next() {
        match c {
            '\'' | '"' => {
                quoted = true;
                for inner in chars.by_ref() {
                    if inner == c {
                        break;
                    }
                    word.push(inner);
                }
            }
            '\\' => word.extend(chars.next()),
            c if c.is_whitespace() => end(&mut word, &mut quoted, &mut found),
            // `>`, `>>`, `<`, `2>`, `2>&1`, `&>`: a word of their own too
            '>' | '<' => {
                let mut op = if matches!(word.as_str(), "1" | "2" | "&") {
                    std::mem::take(&mut word)
                } else {
                    end(&mut word, &mut quoted, &mut found);
                    String::new()
                };
                op.push(c);
                if chars.peek() == Some(&c) {
                    op.push(c);
                    chars.next();
                }
                if chars.peek() == Some(&'&') {
                    op.push('&');
                    chars.next();
                    while let Some(digit) = chars.next_if(char::is_ascii_digit) {
                        op.push(digit);
                    }
                }
                found.push(op);
            }
            '&' if chars.peek() == Some(&'>') => word.push('&'), // `&>`
            '|' | '&' | ';' => {
                end(&mut word, &mut quoted, &mut found);
                let mut op = c.to_string();
                if c != ';' && chars.peek() == Some(&c) {
                    op.push(c); // `||`, `&&`
                    chars.next();
                }
                found.push(op);
            }
            c => word.push(c),
        }
    }
    end(&mut word, &mut quoted, &mut found);
    found
}

/// The options a man page lists: a line that starts, a little indented,
/// with `-` (`-f, --file=ARCHIVE`, `-l     use a long listing format`),
/// and its description on the same line or the more indented lines under it.
pub fn options_in_man(lines: &[String]) -> Vec<ManOption> {
    let mut found = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let indent = line.len() - line.trim_start().len();
        let trimmed = line.trim();
        let starts_option = trimmed.starts_with('-')
            && trimmed[1..].starts_with(|c: char| c.is_ascii_alphanumeric() || c == '-');
        if indent == 0 || indent > 12 || !starts_option {
            continue;
        }
        // the spellings end at a run of spaces (or a tab): the text follows
        let (spec, rest) = match trimmed.find("  ").or_else(|| trimmed.find('\t')) {
            Some(gap) => (&trimmed[..gap], trimmed[gap..].trim()),
            None => (trimmed, ""),
        };
        let mut names = Vec::new();
        let mut takes_value = false;
        for part in spec.split(", ") {
            let part = part.trim();
            if !part.starts_with('-') {
                continue;
            }
            let end = part.find(['=', '[', ' ']).unwrap_or(part.len());
            names.push(part[..end].to_owned());
            let after = &part[end..];
            takes_value |= after.starts_with('=') || after.starts_with(' ');
        }
        if names.is_empty() {
            continue;
        }
        let mut text = rest.to_owned();
        if text.is_empty() {
            // the description is under it, more indented, up to a blank line
            for next in &lines[i + 1..] {
                let next_indent = next.len() - next.trim_start().len();
                if next.trim().is_empty() || next_indent <= indent {
                    break;
                }
                text.push(' ');
                text.push_str(next.trim());
            }
        }
        found.push(ManOption {
            names,
            takes_value,
            text: first_sentence(&text),
        });
    }
    found
}

/// The first sentence of `text`, its spaces evened out (man justifies
/// lines with double spaces), at most `EXPLAIN_MAX_CHARS` long.
fn first_sentence(text: &str) -> String {
    let even = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let sentence = match even.find(". ") {
        Some(end) => &even[..=end],
        None => &even,
    };
    if sentence.chars().count() <= EXPLAIN_MAX_CHARS {
        return sentence.to_owned();
    }
    let cut: String = sentence.chars().take(EXPLAIN_MAX_CHARS).collect();
    let cut = cut.rsplit_once(' ').map_or(cut.as_str(), |(head, _)| head);
    format!("{cut}...")
}
