//! Colours a line of code the way `bat` would, roughly, for any of the
//! languages of the languages topic: comments (line and block), strings,
//! numbers and the keywords most languages share. One small table
//! (`LANGUAGE_SYNTAXES`) instead of a grammar per language: these pages are
//! read, not compiled, and the comments are most of them.

use crate::compile_config::{LANGUAGE_GUESSED_COMMENTS, LANGUAGE_KEYWORDS, LANGUAGE_SYNTAXES};
use crate::info::page::Style;

/// How one language writes comments and strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Syntax {
    pub line: Vec<&'static str>,
    pub blocks: Vec<(&'static str, &'static str)>,
    /// Whether '…' and "…" are strings.
    pub single: bool,
    pub double: bool,
}

/// The syntax for a code block's tag (`python`) or a file's extension
/// (`py`), if the table has it.
pub fn by_name(name: &str) -> Option<Syntax> {
    let name = name.trim().to_lowercase();
    LANGUAGE_SYNTAXES
        .iter()
        .find(|(names, ..)| names.contains(&name.as_str()))
        .map(|(_, line, blocks, single, double)| Syntax {
            line: line.to_vec(),
            blocks: blocks.to_vec(),
            single: *single,
            double: *double,
        })
}

/// A syntax guessed from the code itself: the comment mark most lines
/// start with, `/* */` blocks when the code has them.
pub fn guess(code: &[String]) -> Syntax {
    let starting = |mark: &str| {
        code.iter()
            .filter(|line| line.trim_start().starts_with(mark))
            .count()
    };
    let best = LANGUAGE_GUESSED_COMMENTS
        .iter()
        .copied()
        .max_by_key(|mark| starting(mark))
        .filter(|mark| starting(mark) > 0);
    let blocks = if code.iter().any(|line| line.contains("/*")) {
        vec![("/*", "*/")]
    } else {
        Vec::new()
    };
    Syntax {
        line: best.into_iter().collect(),
        blocks,
        single: best != Some("'"),
        double: true,
    }
}

/// `code`, one line, in coloured parts. `open` is the end of a block
/// comment still open from the lines before (None: none), and is left as
/// this line leaves it.
pub fn line(code: &str, syntax: &Syntax, open: &mut Option<&'static str>) -> Vec<(String, Style)> {
    let mut parts: Vec<(String, Style)> = Vec::new();
    let mut push = |text: &str, style: Style| {
        if text.is_empty() {
            return;
        }
        match parts.last_mut() {
            Some((last, last_style)) if *last_style == style => last.push_str(text),
            _ => parts.push((text.to_owned(), style)),
        }
    };
    let mut rest = code;
    // inside a block comment from before: up to its end
    if let Some(close) = *open {
        match rest.find(close) {
            Some(at) => {
                push(&rest[..at + close.len()], Style::Comment);
                rest = &rest[at + close.len()..];
                *open = None;
            }
            None => {
                push(rest, Style::Comment);
                return parts;
            }
        }
    }
    let mut before: Option<char> = None; // the character before `rest`
    while let Some(c) = rest.chars().next() {
        // a comment may start after a space or a bracket, not inside a word
        // (`$#` in a shell, `a--` in C are not comments)
        let may_comment = before.is_none_or(|b| b.is_whitespace() || "([{;,)}]".contains(b));
        if may_comment {
            if let Some((start, close)) = syntax
                .blocks
                .iter()
                .find(|(start, _)| rest.starts_with(start))
            {
                match rest[start.len()..].find(close) {
                    Some(at) => {
                        let end = start.len() + at + close.len();
                        push(&rest[..end], Style::Comment);
                        before = rest[..end].chars().last();
                        rest = &rest[end..];
                    }
                    None => {
                        push(rest, Style::Comment);
                        *open = Some(close);
                        return parts;
                    }
                }
                continue;
            }
            if syntax.line.iter().any(|mark| rest.starts_with(mark)) {
                push(rest, Style::Comment);
                return parts;
            }
        }
        let quote = (c == '"' && syntax.double) || (c == '\'' && syntax.single);
        if quote {
            if let Some(end) = string_end(rest, c) {
                push(&rest[..end], Style::Literal);
                before = Some(c);
                rest = &rest[end..];
                continue;
            }
            if c == '"' {
                push(rest, Style::Literal); // a string that goes on: the rest of the line
                return parts;
            }
        }
        let after_word = before.is_some_and(|b| b.is_alphanumeric() || b == '_');
        if c.is_ascii_digit() && !after_word {
            let end = rest
                .find(|n: char| !(n.is_alphanumeric() || n == '.' || n == '_'))
                .unwrap_or(rest.len());
            push(&rest[..end], Style::Number);
            before = rest[..end].chars().last();
            rest = &rest[end..];
            continue;
        }
        if (c.is_alphabetic() || c == '_') && !after_word {
            let end = rest
                .find(|n: char| !(n.is_alphanumeric() || n == '_'))
                .unwrap_or(rest.len());
            let word = &rest[..end];
            let style = if LANGUAGE_KEYWORDS.contains(&word) {
                Style::Keyword
            } else {
                Style::Text
            };
            push(word, style);
            before = word.chars().last();
            rest = &rest[end..];
            continue;
        }
        let size = c.len_utf8();
        push(&rest[..size], Style::Text);
        before = Some(c);
        rest = &rest[size..];
    }
    parts
}

/// Where the string `text` starts with ends: after its closing `quote`,
/// past backslash escapes; None when it does not close on this line.
fn string_end(text: &str, quote: char) -> Option<usize> {
    let mut escaped = false;
    for (at, c) in text.char_indices().skip(1) {
        match c {
            '\\' if !escaped => escaped = true,
            c if c == quote && !escaped => return Some(at + c.len_utf8()),
            _ => escaped = false,
        }
    }
    None
}
