//! Hardcoded values: the docs topic (src/info/docs.rs, src/utils/html.rs).

/// Every documentation set DevDocs offers: devdocs/catalog.tsv, from
/// scripts/docs/catalog.py, ranked by Stack Overflow's most asked-about.
pub const DOCS_CATALOG: &str = include_str!("../../devdocs/catalog.tsv");
/// A set's two files on DevDocs, fetched when the reader asks (`d`).
pub const DOCS_INDEX_URL: &str = "https://documents.devdocs.io/{slug}/index.json?{mtime}";
pub const DOCS_DB_URL: &str = "https://documents.devdocs.io/{slug}/db.json?{mtime}";
/// The largest file a set may have (OpenJDK's is about 120 MB).
pub const DOCS_MAX_BYTES: usize = 300 * 1024 * 1024;
/// Where downloaded sets are kept, in the data folder (`~/.local/share/lore/docs`).
pub const DOCS_DIR: &str = "docs";
/// How many sets "most searched" lists.
pub const DOCS_MOST_SEARCHED: usize = 50;
/// The keys, in the docs topic: download a set, delete it.
pub const DOWNLOAD_KEY: char = 'd';
pub const DELETE_KEY: char = 'x';
/// The docs topic's headings.
pub const DOCS_DOWNLOADED: &str = "downloaded";
pub const DOCS_MOST: &str = "most searched (Stack Overflow)";
pub const DOCS_ALL: &str = "every set on devdocs.io";
/// After each set's name in the list.
pub const DOCS_HERE: &str = "on this machine · x deletes";
pub const DOCS_SIZE: &str = "{size} MB · d downloads";
pub const DOCS_DOWNLOADING: &str = "downloading…";
pub const DOCS_FAILED: &str = "failed: {why}";
/// A set's own page, and an entry's.
pub const DOCS_INFO_SIZE: &str = "{size} MB of documentation from devdocs.io, to read offline.";
pub const DOCS_INFO_DOWNLOAD: &str = "Press d to download it; Enter on it then lists its pages.";
pub const DOCS_INFO_HERE: &str = "On this machine: Enter lists its pages, x deletes it.";
pub const DOCS_FROM: &str = "From devdocs.io. {who}";
pub const DOCS_NO_PAGE: &str = "This page is not in the download.";
/// The key bar's note after `d` and `x`.
pub const DOCS_STARTED: &str = "downloading {name} in the background…";
pub const DOCS_DELETED: &str = "deleted {name} from this machine";
pub const DOCS_NOT_DELETED: &str = "could not delete {name}: {why}";
/// The tools topic, when no tool page is there (all of them filtered out).
pub const TOOLS_EMPTY: &str = "No tool pages here.";
