//! The docs topic: documentation sets for languages and tools, from DevDocs
//! (https://devdocs.io). Which sets there are is built in
//! (devdocs/catalog.tsv, from scripts/docs/catalog.py, ranked by how much
//! Stack Overflow asks about each); a set's pages are downloaded only when
//! the reader asks (`d`), kept in `$XDG_DATA_HOME/lore/docs/<set>/` to read
//! offline, and deleted when asked (`x`).
//!
//! DevDocs gives a set as two files: `index.json`, its entries (name, path,
//! type), and `db.json`, each path's HTML. lore keeps them as:
//!
//! ```text
//! docs/python~3.13/index.tsv   name<TAB>path<TAB>type, one entry a line
//! docs/python~3.13/pages       every page's HTML, one after the other
//! docs/python~3.13/offsets.tsv path<TAB>start<TAB>length, into pages
//! ```

use std::collections::HashMap;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::compile_config::{
    DOCS_CATALOG, DOCS_DB_URL, DOCS_DIR, DOCS_FROM, DOCS_INDEX_URL, DOCS_INFO_DOWNLOAD,
    DOCS_INFO_HERE, DOCS_INFO_SIZE, DOCS_MAX_BYTES, DOCS_NO_PAGE, INTERNET_TIMEOUT,
};
use crate::info::page::{PageLine, Style};
use crate::settings;
use crate::utils::{html, internet, text};

/// One documentation set of the catalogue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Docset {
    /// DevDocs' name for it (`python~3.13`): its folder and its address.
    pub slug: String,
    pub name: String,
    pub version: String,
    pub bytes: u64,
    pub mtime: String,
    /// Its rank among the most asked-about on Stack Overflow; 0: not ranked.
    pub rank: u32,
    pub home: String,
    pub attribution: String,
}

impl Docset {
    /// How the list shows it: `Python 3.13`, or `Bash`.
    pub fn title(&self) -> String {
        if self.version.is_empty() {
            self.name.clone()
        } else {
            format!("{} {}", self.name, self.version)
        }
    }
}

/// The catalogue's sets, as DevDocs lists them (newest version first).
pub fn catalog() -> Vec<Docset> {
    parse_catalog(DOCS_CATALOG)
}

/// The catalogue's text: `slug name version release bytes mtime rank home
/// attribution`, tab-separated; comments and broken lines skipped.
pub fn parse_catalog(text: &str) -> Vec<Docset> {
    text.lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| {
            let fields: Vec<&str> = line.split('\t').collect();
            let [
                slug,
                name,
                version,
                _release,
                bytes,
                mtime,
                rank,
                home,
                attribution,
            ] = fields.as_slice()
            else {
                return None;
            };
            let safe = !slug.is_empty()
                && !slug.starts_with('.')
                && slug
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "~._-".contains(c));
            safe.then(|| Docset {
                slug: (*slug).to_owned(),
                name: (*name).to_owned(),
                version: (*version).to_owned(),
                bytes: bytes.parse().unwrap_or(0),
                mtime: (*mtime).to_owned(),
                rank: rank.parse().unwrap_or(0),
                home: (*home).to_owned(),
                attribution: (*attribution).to_owned(),
            })
        })
        .collect()
}

/// The most asked-about sets, one version of each (the newest), most
/// asked first.
pub fn most_searched(sets: &[Docset], most: usize) -> Vec<&Docset> {
    let mut seen = std::collections::HashSet::new();
    let mut ranked: Vec<&Docset> = sets
        .iter()
        .filter(|set| set.rank > 0)
        .filter(|set| seen.insert(set.slug.split('~').next().unwrap_or_default().to_owned()))
        .collect();
    ranked.sort_by_key(|set| (set.rank, set.title()));
    ranked.truncate(most);
    ranked
}

/// `$XDG_DATA_HOME/lore/docs` (or `~/.local/share/lore/docs`).
pub fn folder() -> Option<PathBuf> {
    settings::list_path(DOCS_DIR)
}

fn set_folder(slug: &str) -> Option<PathBuf> {
    Some(folder()?.join(slug))
}

/// Whether `slug` was downloaded (and finished).
pub fn is_downloaded(slug: &str) -> bool {
    set_folder(slug).is_some_and(|dir| dir.join("offsets.tsv").is_file())
}

/// Deletes a downloaded set; nothing there is no error.
pub fn delete(slug: &str) -> std::io::Result<()> {
    match set_folder(slug) {
        Some(dir) if dir.exists() => fs::remove_dir_all(dir),
        _ => Ok(()),
    }
}

/// One entry of a set: what it is called, where its page is (`path`, maybe
/// with `#part`), and its type (DevDocs' grouping: `Built-in Functions`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub path: String,
    pub kind: String,
}

/// A downloaded set's entries, in DevDocs' order; empty if not there.
pub fn entries(slug: &str) -> Vec<Entry> {
    let Some(text) =
        set_folder(slug).and_then(|dir| fs::read_to_string(dir.join("index.tsv")).ok())
    else {
        return Vec::new();
    };
    text.lines()
        .filter_map(|line| {
            let mut fields = line.split('\t');
            Some(Entry {
                name: fields.next()?.to_owned(),
                path: fields.next()?.to_owned(),
                kind: fields.next().unwrap_or_default().to_owned(),
            })
        })
        .collect()
}

/// The HTML of `path` (its `#part` ignored) in a downloaded set.
pub fn page_html(slug: &str, path: &str) -> Option<String> {
    let dir = set_folder(slug)?;
    let page = path.split('#').next().unwrap_or(path);
    let offsets = fs::read_to_string(dir.join("offsets.tsv")).ok()?;
    let (start, length) = offsets.lines().find_map(|line| {
        let mut fields = line.split('\t');
        (fields.next()? == page).then_some(())?;
        Some((
            fields.next()?.parse::<u64>().ok()?,
            fields.next()?.parse::<u64>().ok()?,
        ))
    })?;
    let mut file = fs::File::open(dir.join("pages")).ok()?;
    file.seek(SeekFrom::Start(start)).ok()?;
    let mut bytes = Vec::new();
    file.take(length).read_to_end(&mut bytes).ok()?;
    String::from_utf8(bytes).ok()
}

/// What a set's own page says, before it is downloaded or as its cover:
/// its name, size, where it is from, and how to get or remove it.
pub fn set_page(set: &Docset, width: usize) -> Vec<PageLine> {
    let mut lines = vec![line(&set.title(), Style::Title), line("", Style::Text)];
    let size = format!("{:.1}", set.bytes as f64 / 1_048_576.0);
    for text in [
        DOCS_INFO_SIZE.replace("{size}", &size),
        if is_downloaded(&set.slug) {
            DOCS_INFO_HERE.to_owned()
        } else {
            DOCS_INFO_DOWNLOAD.to_owned()
        },
    ] {
        lines.extend(
            text::wrap(&text, width)
                .into_iter()
                .map(|t| line(&t, Style::Text)),
        );
    }
    lines.push(line("", Style::Text));
    if set.home.starts_with("https://") {
        lines.push(line(&set.home, Style::Url));
    }
    for text in text::wrap(&DOCS_FROM.replace("{who}", &set.attribution), width) {
        lines.push(line(&text, Style::Dim));
    }
    lines
}

/// An entry's page: its title, its set, then its HTML as text.
pub fn entry_page(set: &Docset, entry: &Entry, width: usize) -> Vec<PageLine> {
    let mut lines = vec![
        line(&entry.name, Style::Title),
        line(
            &format!("[{}] {}", set.title(), entry.kind),
            Style::Categories,
        ),
        line("", Style::Text),
    ];
    match page_html(&set.slug, &entry.path) {
        Some(page) => lines.extend(html::to_lines(from_anchor(&page, &entry.path), width)),
        None => lines.push(line(DOCS_NO_PAGE, Style::Dim)),
    }
    lines.push(line("", Style::Text));
    for text in text::wrap(&DOCS_FROM.replace("{who}", &set.attribution), width) {
        lines.push(line(&text, Style::Dim));
    }
    lines
}

/// The page from the entry's own part on (`path#part`: the element with
/// that id), so it opens where the entry is; the whole page without one.
fn from_anchor<'a>(page: &'a str, path: &str) -> &'a str {
    let Some((_, part)) = path.split_once('#') else {
        return page;
    };
    let id = [format!("id=\"{part}\""), format!("id='{part}'")];
    id.iter()
        .find_map(|id| page.find(id.as_str()))
        .and_then(|at| page[..at].rfind('<'))
        .map_or(page, |start| &page[start..])
}

fn line(text: &str, style: Style) -> PageLine {
    PageLine {
        text: text.to_owned(),
        style,
        spans: Vec::new(),
    }
}

/// Where a download is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Progress {
    Downloading,
    Failed(String),
}

/// Downloads under way (or failed), shared with their threads; `changed`
/// asks the screen to draw again.
#[derive(Debug, Default)]
pub struct Downloads {
    state: Mutex<HashMap<String, Progress>>,
    changed: AtomicBool,
}

impl Downloads {
    pub fn progress(&self, slug: &str) -> Option<Progress> {
        self.state.lock().ok()?.get(slug).cloned()
    }

    /// True once after a download ended (or failed).
    pub fn take_changed(&self) -> bool {
        self.changed.swap(false, Ordering::Relaxed)
    }

    /// Downloads `set` in the background, unless it is under way already.
    pub fn start(self: &Arc<Self>, set: Docset) {
        let Ok(mut state) = self.state.lock() else {
            return;
        };
        if state.get(&set.slug) == Some(&Progress::Downloading) {
            return;
        }
        state.insert(set.slug.clone(), Progress::Downloading);
        drop(state);
        let shared = Arc::clone(self);
        thread::spawn(move || {
            let result = download(&set);
            if let Ok(mut state) = shared.state.lock() {
                match result {
                    Ok(()) => state.remove(&set.slug),
                    Err(why) => state.insert(set.slug.clone(), Progress::Failed(why)),
                };
            }
            shared.changed.store(true, Ordering::Relaxed);
        });
    }
}

/// Fetches a set's two files and keeps them as lore reads them; written
/// beside the folder first, so a failure leaves nothing half done.
pub fn download(set: &Docset) -> Result<(), String> {
    let dir = set_folder(&set.slug).ok_or("no home folder")?;
    let wait = INTERNET_TIMEOUT * 100; // up to a hundred megabytes, on a slow line
    let url = |template: &str| {
        template
            .replace("{slug}", &set.slug)
            .replace("{mtime}", &set.mtime)
    };
    let index = internet::download(&url(DOCS_INDEX_URL), DOCS_MAX_BYTES, wait)?;
    let db = internet::download(&url(DOCS_DB_URL), DOCS_MAX_BYTES, wait)?;
    let index: serde_json::Value = serde_json::from_slice(&index).map_err(|e| e.to_string())?;
    let db: serde_json::Value = serde_json::from_slice(&db).map_err(|e| e.to_string())?;
    let entries = index["entries"].as_array().ok_or("no entries")?;
    let pages = db.as_object().ok_or("no pages")?;

    let fresh = dir.with_extension("part");
    let _ = fs::remove_dir_all(&fresh);
    fs::create_dir_all(&fresh).map_err(|e| e.to_string())?;
    let clean = |text: &str| text.replace(['\t', '\n', '\r'], " ");
    let mut list = String::new();
    for entry in entries {
        let (Some(name), Some(path)) = (entry["name"].as_str(), entry["path"].as_str()) else {
            continue;
        };
        let kind = entry["type"].as_str().unwrap_or_default();
        list.push_str(&format!(
            "{}\t{}\t{}\n",
            clean(name),
            clean(path),
            clean(kind)
        ));
    }
    let mut all = String::new();
    let mut offsets = String::new();
    for (path, page) in pages {
        let Some(page) = page.as_str() else { continue };
        offsets.push_str(&format!("{}\t{}\t{}\n", clean(path), all.len(), page.len()));
        all.push_str(page);
    }
    let write =
        |file: &str, text: &str| fs::write(fresh.join(file), text).map_err(|e| e.to_string());
    write("index.tsv", &list)?;
    write("pages", &all)?;
    write("offsets.tsv", &offsets)?; // last: its presence means done
    let _ = fs::remove_dir_all(&dir);
    fs::rename(&fresh, &dir).map_err(|e| e.to_string())
}
