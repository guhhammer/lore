//! HTML, as lore shows it on a page: the docs topic's pages (DevDocs) are
//! HTML. Text in paragraphs, wrapped to the panel; headings as headings
//! (the jumper stops at them); `<pre>` blocks as code, line by line; list
//! items with a bullet; table cells side by side. Scripts and styles are
//! dropped, entities decoded, and control and invisible characters taken
//! out, as for any text from outside.

use crate::info::content::shown;
use crate::info::page::{PageLine, Style};
use crate::utils::text;

/// `html` as page lines, `width` columns wide.
pub fn to_lines(html: &str, width: usize) -> Vec<PageLine> {
    let mut out = Writer {
        lines: Vec::new(),
        text: String::new(),
        heading: false,
        bullet: None,
        width: width.max(10),
    };
    let mut rest = html;
    let mut pre: Option<String> = None; // the text of a <pre> being read
    let mut skip: Option<String> = None; // inside <script> or <style>
    let mut lists = 0usize;
    while !rest.is_empty() {
        let Some(open) = rest.find('<') else {
            out.add_text(rest, pre.as_mut());
            break;
        };
        if skip.is_none() {
            out.add_text(&rest[..open], pre.as_mut()); // a script's text is never shown
        }
        rest = &rest[open..];
        if let Some(after) = rest.strip_prefix("<!--") {
            rest = after.find("-->").map_or("", |end| &after[end + 3..]);
            continue;
        }
        let Some(close) = rest.find('>') else {
            break; // a tag cut off: nothing more to read
        };
        let tag = &rest[1..close];
        rest = &rest[close + 1..];
        let closing = tag.starts_with('/');
        let name: String = tag
            .trim_start_matches('/')
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric())
            .collect::<String>()
            .to_ascii_lowercase();
        if let Some(skipped) = &skip {
            if closing && name == *skipped {
                skip = None;
            }
            continue;
        }
        match (name.as_str(), closing) {
            ("script" | "style" | "template", false) => skip = Some(name),
            ("pre", false) => {
                out.flush();
                pre = Some(String::new());
            }
            ("pre", true) => {
                if let Some(code) = pre.take() {
                    out.code(&code);
                }
            }
            _ if pre.is_some() => {
                if name == "br"
                    && let Some(code) = pre.as_mut()
                {
                    code.push('\n')
                }
            }
            ("h1" | "h2" | "h3" | "h4" | "h5" | "h6", false) => {
                out.flush();
                out.heading = true;
            }
            ("h1" | "h2" | "h3" | "h4" | "h5" | "h6", true) => out.flush(),
            ("ul" | "ol", false) => {
                out.flush();
                lists += 1;
            }
            ("ul" | "ol", true) => {
                out.flush();
                lists = lists.saturating_sub(1);
            }
            ("li", false) => {
                out.flush();
                out.bullet = Some("  ".repeat(lists.saturating_sub(1)) + "• ");
            }
            ("td" | "th", false) => {
                if !out.text.trim().is_empty() {
                    out.text.push_str("  │ ");
                }
            }
            ("br" | "hr", _) => out.flush(),
            (
                "p" | "div" | "section" | "article" | "header" | "footer" | "table" | "tr"
                | "blockquote" | "dl" | "dt" | "dd" | "details" | "summary" | "figure"
                | "figcaption" | "aside" | "nav" | "main" | "li",
                _,
            ) => out.flush(),
            _ => {}
        }
    }
    if let Some(code) = pre.take() {
        out.code(&code);
    }
    out.flush();
    while out.lines.last().is_some_and(|l| l.text.is_empty()) {
        out.lines.pop();
    }
    out.lines
}

/// The `<pre>` blocks of `html`, each as typed, for `c` to copy.
pub fn code_blocks(html: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut block: Vec<String> = Vec::new();
    // laid out wide enough that no code line is cut
    for line in to_lines(html, usize::MAX / 4).into_iter().chain([PageLine {
        text: String::new(),
        style: Style::Text,
        spans: Vec::new(),
    }]) {
        if line.style == Style::Code {
            block.push(
                line.text
                    .strip_prefix("  ")
                    .unwrap_or(&line.text)
                    .to_owned(),
            );
        } else if !block.is_empty() {
            blocks.push(std::mem::take(&mut block).join("\n"));
        }
    }
    blocks
}

/// The lines made so far, and the paragraph being read.
struct Writer {
    lines: Vec<PageLine>,
    text: String,
    /// The paragraph is a heading.
    heading: bool,
    /// The paragraph is a list item: its bullet, indented by depth.
    bullet: Option<String>,
    width: usize,
}

impl Writer {
    fn push(&mut self, text: String, style: Style) {
        self.lines.push(PageLine {
            text,
            style,
            spans: Vec::new(),
        });
    }

    /// Text between tags: into the `<pre>` being read as it is, else into
    /// the paragraph with its spaces joined.
    fn add_text(&mut self, raw: &str, pre: Option<&mut String>) {
        let decoded = decode(raw);
        match pre {
            Some(code) => code.push_str(&decoded),
            None => {
                if decoded.starts_with(char::is_whitespace) && !self.text.ends_with(' ') {
                    self.text.push(' ');
                }
                self.text
                    .push_str(&decoded.split_whitespace().collect::<Vec<_>>().join(" "));
                if decoded.ends_with(char::is_whitespace) && !self.text.is_empty() {
                    self.text.push(' ');
                }
            }
        }
    }

    /// Ends the paragraph: wrapped, after a blank line from the one before.
    fn flush(&mut self) {
        let words = self.text.trim().to_owned();
        self.text.clear();
        let heading = std::mem::take(&mut self.heading);
        let bullet = self.bullet.take();
        if words.is_empty() {
            return;
        }
        let tight = bullet.is_some()
            && self
                .lines
                .last()
                .is_some_and(|l| l.text.starts_with([' ', '•']));
        if !tight && self.lines.last().is_some_and(|l| !l.text.is_empty()) {
            self.push(String::new(), Style::Text);
        }
        let style = if heading { Style::Heading } else { Style::Text };
        let lead = bullet.unwrap_or_default();
        let room = self.width.saturating_sub(text::width(&lead)).max(10);
        let indent = " ".repeat(text::width(&lead));
        for (i, line) in text::wrap(&words, room).into_iter().enumerate() {
            let start = if i == 0 { &lead } else { &indent };
            self.push(format!("{start}{line}"), style);
        }
    }

    /// A `<pre>` block: each line as code, cut into rows of the width.
    fn code(&mut self, code: &str) {
        self.flush();
        if self.lines.last().is_some_and(|l| !l.text.is_empty()) {
            self.push(String::new(), Style::Text);
        }
        let room = self.width.saturating_sub(2).max(10);
        for line in code.trim_matches('\n').lines() {
            let mut rest = line.replace('\t', "    ");
            loop {
                let (piece, after) = text::take_columns(&rest, room);
                let (piece, after) = (piece.to_owned(), after.to_owned());
                self.push(format!("  {piece}"), Style::Code);
                if after.is_empty() {
                    break;
                }
                rest = after;
            }
        }
        self.push(String::new(), Style::Text);
    }
}

/// Entities decoded (`&lt;` → `<`, `&#39;` → `'`), and characters a
/// terminal could obey or that hide text taken out.
pub fn decode(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find('&') {
        out.push_str(&rest[..at]);
        rest = &rest[at..];
        let end = rest.find(';').filter(|&end| end <= 10);
        let decoded = end.and_then(|end| entity(&rest[1..end]));
        match (end, decoded) {
            (Some(end), Some(c)) => {
                out.push(c);
                rest = &rest[end + 1..];
            }
            _ => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out.chars()
        .filter(|&c| c == '\n' || c == '\t' || shown(c))
        .collect()
}

/// The character an entity's name (`lt`, `#39`, `#x27`) stands for.
fn entity(name: &str) -> Option<char> {
    if let Some(number) = name.strip_prefix('#') {
        let code = match number.strip_prefix(['x', 'X']) {
            Some(hex) => u32::from_str_radix(hex, 16).ok()?,
            None => number.parse().ok()?,
        };
        return char::from_u32(code);
    }
    Some(match name {
        "amp" => '&',
        "lt" => '<',
        "gt" => '>',
        "quot" => '"',
        "apos" => '\'',
        "nbsp" => ' ',
        "mdash" => '—',
        "ndash" => '–',
        "hellip" => '…',
        "copy" => '©',
        "reg" => '®',
        "trade" => '™',
        "rarr" => '→',
        "larr" => '←',
        "uarr" => '↑',
        "darr" => '↓',
        "times" => '×',
        "laquo" => '«',
        "raquo" => '»',
        "lsquo" => '‘',
        "rsquo" => '’',
        "ldquo" => '“',
        "rdquo" => '”',
        "bull" => '•',
        "middot" => '·',
        "deg" => '°',
        "para" => '¶',
        "sect" => '§',
        _ => return None,
    })
}
