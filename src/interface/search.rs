//! The search rules: what matches and in which order. With a query the
//! list is ranked, best first: names (exact, then starting with it…), then
//! pages. Names are matched as typed; a longer query also searches what the pages say (their
//! categories, summary and examples), so "compress folder" finds tar and
//! zip. A query starting with `@` keeps only the commands that have a page.

use crate::compile_config::SYNONYMS;
use crate::info::content::Content;

/// Starts a query that keeps only the commands with a lore page.
pub const PAGES_ONLY: char = '@';

/// (only commands with a lore page?, the name part): `@gr` → (true, "gr").
pub fn split(query: &str) -> (bool, &str) {
    match query.strip_prefix(PAGES_ONLY) {
        Some(rest) => (true, rest),
        None => (false, query),
    }
}

/// How well `name` matches `query` by name, lower first: exact, then
/// starting with it, then with it at the start of a word inside the name
/// (`git commit` for "commit", `apt-get` for "get"), then anywhere; within
/// each, shorter names and earlier matches first. None if it does not
/// contain the query at all.
pub fn name_rank(name: &str, query: &str) -> Option<(u8, usize, usize)> {
    let name = name.to_lowercase();
    let query = query.to_lowercase();
    let position = name.find(&query)?;
    let at_word_start = name
        .match_indices(&query)
        .any(|(i, _)| i > 0 && name[..i].ends_with([' ', '-', '_', '.', ':']));
    let tier = if name == query {
        0
    } else if position == 0 {
        1
    } else if at_word_start {
        2
    } else {
        3
    };
    Some((tier, name.chars().count(), position))
}

/// A query needs this many characters (spaces not counted) before pages are
/// searched too: fewer would match nearly every page.
pub const CONTENT_MIN_CHARS: usize = 3;

/// What a page offers the search, lower-cased once.
#[derive(Debug, Clone, Default)]
pub struct Doc {
    pub categories: Vec<String>,
    /// The page's lines as written, for showing the one that matched:
    /// summary paragraphs, then the examples' descriptions, then examples.
    lines: Vec<String>,
    summary: String,
    descriptions: String,
    examples: String,
}

impl Doc {
    pub fn of(content: &Content) -> Self {
        let lower = |parts: Vec<&str>| parts.join(" \n ").to_lowercase();
        let lines = content
            .summary
            .iter()
            .cloned()
            .chain(content.usage.iter().map(|(_, what)| what.clone()))
            .chain(content.usage.iter().map(|(typed, _)| typed.clone()))
            .collect();
        Self {
            categories: content.categories.clone(),
            lines,
            summary: lower(content.summary.iter().map(String::as_str).collect()),
            descriptions: lower(
                content
                    .usage
                    .iter()
                    .map(|(_, what)| what.as_str())
                    .collect(),
            ),
            examples: lower(
                content
                    .usage
                    .iter()
                    .map(|(typed, _)| typed.as_str())
                    .collect(),
            ),
        }
    }
}

/// How well a page matches `query`: 0 when some word of it is nowhere on
/// the page; else higher for words in its categories, then its summary,
/// then its examples' descriptions, then the examples, plus a bonus when the
/// whole query appears as typed.
pub fn content_score(doc: &Doc, query: &str) -> u32 {
    let query = query.trim().to_lowercase();
    if query.chars().filter(|c| !c.is_whitespace()).count() < CONTENT_MIN_CHARS {
        return 0;
    }
    let mut score = 0;
    for word in query.split_whitespace() {
        // the word as typed counts most; a word that means the same, a bit less
        let found = word_score(doc, word) * 2
            + alternatives(word)
                .iter()
                .map(|other| word_score(doc, other))
                .max()
                .unwrap_or(0);
        if found == 0 {
            return 0; // every word (or one meaning the same) must be somewhere
        }
        score += found;
    }
    if query.contains(' ')
        && (find_word(&doc.summary, &query).is_some()
            || find_word(&doc.descriptions, &query).is_some())
    {
        score += 5;
    }
    score
}

/// Where on the page `word` is: categories count most, then the summary,
/// then the examples' descriptions, then the examples themselves.
fn word_score(doc: &Doc, word: &str) -> u32 {
    let mut found = 0;
    if doc.categories.iter().any(|c| c == word) {
        found += 6;
    } else if doc.categories.iter().any(|c| c.contains(word)) {
        found += 4;
    }
    if find_word(&doc.summary, word).is_some() {
        found += 3;
    }
    if find_word(&doc.descriptions, word).is_some() {
        found += 2;
    }
    if find_word(&doc.examples, word).is_some() {
        found += 1;
    }
    found
}

/// Where `word` starts a word in `text` (lower-cased both): "cut" is in
/// "cuts" and "cut-off", but not in "execute".
pub fn find_word(text: &str, word: &str) -> Option<usize> {
    text.match_indices(word).map(|(at, _)| at).find(|&at| {
        !text[..at]
            .chars()
            .next_back()
            .is_some_and(char::is_alphanumeric)
    })
}

/// Whether `name` is one of the query's words: "cut files" names `cut`,
/// which then goes first among the results found by their pages.
pub fn names_a_word(name: &str, query: &str) -> bool {
    let name = name.to_lowercase();
    query
        .to_lowercase()
        .split_whitespace()
        .any(|word| word == name)
}

/// Words that count as `word`: the rest of its `SYNONYMS` groups, and its
/// singular when it looks plural ("files" → "file", "directories" →
/// "directory"), so a search need not guess the page's wording.
pub fn alternatives(word: &str) -> Vec<String> {
    let mut found: Vec<String> = SYNONYMS
        .iter()
        .filter(|group| group.contains(&word))
        .flat_map(|group| group.iter().map(|w| (*w).to_owned()))
        .filter(|other| other != word)
        .collect();
    if word.chars().count() > 3 {
        if let Some(stem) = word.strip_suffix("ies") {
            found.push(format!("{stem}y"));
        } else if let Some(stem) = word.strip_suffix('s').filter(|s| !s.ends_with('s')) {
            found.push(stem.to_owned());
        }
    }
    found
}

/// The page line that best matches `query`, cut around the match:
/// `[... cut out fields from a file ...]`. The line with the most query
/// words (or words counting as them) wins; descriptions before examples
/// on a tie. None when no line has any, or the query is too short.
pub fn snippet(doc: &Doc, query: &str) -> Option<String> {
    let query = query.trim().to_lowercase();
    if query.chars().filter(|c| !c.is_whitespace()).count() < CONTENT_MIN_CHARS {
        return None;
    }
    // each query word with the words that count as it
    let words: Vec<Vec<String>> = query
        .split_whitespace()
        .map(|w| {
            std::iter::once(w.to_owned())
                .chain(alternatives(w))
                .collect()
        })
        .collect();
    let mut best: Option<(usize, usize, usize, usize)> = None; // (matched, line, first, last)
    for (index, line) in doc.lines.iter().enumerate() {
        // one lower-case char per char, so char positions map back to `line`
        let lower: Vec<char> = line
            .chars()
            .map(|c| c.to_lowercase().next().unwrap_or(c))
            .collect();
        let lower: String = lower.into_iter().collect();
        let mut matched = 0;
        let (mut first, mut last) = (usize::MAX, 0);
        for forms in &words {
            let hit = forms
                .iter()
                .find_map(|form| find_word(&lower, form).map(|at| (at, form)));
            if let Some((at, form)) = hit {
                matched += 1;
                let start = lower[..at].chars().count();
                first = first.min(start);
                last = last.max(start + form.chars().count());
            }
        }
        if matched > 0 && best.is_none_or(|(m, ..)| matched > m) {
            best = Some((matched, index, first, last));
        }
    }
    let (_, index, first, last) = best?;
    let chars: Vec<char> = doc.lines[index].chars().collect();
    // a little context each side, widened to whole words
    let mut start = first.saturating_sub(SNIPPET_CONTEXT);
    while start > 0 && !chars[start - 1].is_whitespace() {
        start -= 1;
    }
    let mut end = (last + SNIPPET_CONTEXT).min(chars.len());
    while end < chars.len() && !chars[end].is_whitespace() {
        end += 1;
    }
    let text: String = chars[start..end].iter().collect();
    let before = if start > 0 { "... " } else { "" };
    let after = if end < chars.len() { " ..." } else { "" };
    Some(format!("[{before}{}{after}]", text.trim()))
}

/// Characters of context kept on each side of a snippet's match.
const SNIPPET_CONTEXT: usize = 12;
