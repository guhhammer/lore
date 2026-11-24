//! Hardcoded values: newer pages: `lore --update-pages` (src/info/updates.rs).

pub const UPDATE_PAGES_FLAG: &str = "--update-pages";
/// First: the pages lore's repository converts each week, one zstd file of
/// records (`RECORD_MARK` + path + newline + the page).
pub const UPDATE_BUNDLE_URL: &str =
    "https://github.com/guhhammer/lore/releases/download/content/pages.zst";
/// A record's start: a control character, which no page that passes the
/// check can hold, so a page cannot fake one.
pub const RECORD_MARK: char = '\u{1e}';
/// Else: tldr-pages' own release, one zip per language, converted here.
pub const UPDATE_TLDR_URL: &str =
    "https://github.com/tldr-pages/tldr/releases/latest/download/tldr-pages.{lang}.zip";
/// tldr's folders this system uses, the first winning a name: the
/// system's own, then the common ones.
pub const TLDR_PLATFORMS_LINUX: &[&str] = &["linux", "common"];
pub const TLDR_PLATFORMS_MACOS: &[&str] = &["osx", "common"];
/// The link every converted page gets, besides tldr's "More information".
pub const TLDR_PAGE_LINK: &str = "https://tldr.inbrowser.app/pages/{platform}/{name}";
pub const TLDR_MORE_INFORMATION: &str = "More information";
/// The most examples a converted page keeps (the built-in pages keep 8 too).
pub const TLDR_MAX_EXAMPLES: usize = 8;
/// The most a download may weigh, and the most a bundle may unpack to.
pub const UPDATE_MAX_BYTES: usize = 64 * 1024 * 1024;
/// What `lore --update-pages` says.
pub const UPDATE_TRYING: &str = "lore: fetching {url}";
pub const UPDATE_FALLBACK: &str = "lore: not there ({why}); converting tldr-pages instead";
pub const UPDATE_DONE: &str =
    "lore: {pages} new pages and {translations} translations in {dir}, from {source}";
pub const UPDATE_SOURCE_BUNDLE: &str = "lore's weekly pages";
pub const UPDATE_SOURCE_TLDR: &str = "tldr-pages (CC BY 4.0)";
pub const ERROR_UPDATE_FAILED: &str = "could not get newer pages: {why}";
pub const ERROR_UPDATE_NO_HOME: &str = "no home folder to keep the pages in";
pub const ERROR_UPDATE_EMPTY: &str = "nothing usable came back";
