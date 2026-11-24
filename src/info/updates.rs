//! `lore --update-pages`: newer pages, only when asked. lore stays offline
//! otherwise. First the pages lore's repository converts each week (one
//! zstd file, `UPDATE_BUNDLE_URL`); if that cannot be had, tldr-pages' own
//! zip files, converted here (`info::tldr`).
//!
//! Only what lore lacks is kept: a page for a command with no built-in
//! page, a translation the built-in page does not have. Every page must
//! pass `info::check`. They go in the first of `UPDATED_CONTENT_DIRS`,
//! replaced whole each time, and are read after the built-in pages, so
//! they never change a page lore already has.

use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::compile_config::{
    ERROR_UPDATE_EMPTY, ERROR_UPDATE_FAILED, ERROR_UPDATE_NO_HOME, INTERNET_TIMEOUT,
    PAGE_MAX_BYTES, RECORD_MARK, TLDR_PLATFORMS_LINUX, TLDR_PLATFORMS_MACOS, UPDATE_BUNDLE_URL,
    UPDATE_DONE, UPDATE_FALLBACK, UPDATE_MAX_BYTES, UPDATE_SOURCE_BUNDLE, UPDATE_SOURCE_TLDR,
    UPDATE_TLDR_URL, UPDATE_TRYING, UPDATED_CONTENT_DIRS,
};
use crate::info::{check, content, tldr};
use crate::interface::paths;
use crate::utils::{internet, zip};

/// One page to keep: its language (None: English), its name, its text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewPage {
    pub lang: Option<String>,
    pub name: String,
    pub text: String,
}

/// Fetches and keeps the newer pages, saying what it does on `say`.
/// Returns the closing line, or why it failed.
pub fn run(say: &mut dyn FnMut(&str)) -> Result<String, String> {
    let dir = folder().ok_or_else(|| ERROR_UPDATE_NO_HOME.to_owned())?;
    let wait = INTERNET_TIMEOUT * 40; // a few megabytes, on a slow line too
    say(&UPDATE_TRYING.replace("{url}", UPDATE_BUNDLE_URL));
    let (pages, source) = match get(UPDATE_BUNDLE_URL, wait).and_then(|b| from_bundle(&b)) {
        Ok(pages) => (pages, UPDATE_SOURCE_BUNDLE),
        Err(why) => {
            say(&UPDATE_FALLBACK.replace("{why}", &why));
            (from_tldr(say, wait)?, UPDATE_SOURCE_TLDR)
        }
    };
    let (new_pages, translations) =
        install(&dir, pages).map_err(|e| ERROR_UPDATE_FAILED.replace("{why}", &e.to_string()))?;
    if new_pages + translations == 0 && source == UPDATE_SOURCE_TLDR {
        return Err(ERROR_UPDATE_FAILED.replace("{why}", ERROR_UPDATE_EMPTY));
    }
    Ok(UPDATE_DONE
        .replace("{pages}", &new_pages.to_string())
        .replace("{translations}", &translations.to_string())
        .replace("{dir}", &dir.display().to_string())
        .replace("{source}", source))
}

/// Where the pages go: the first of `UPDATED_CONTENT_DIRS`.
pub fn folder() -> Option<PathBuf> {
    let home = paths::home();
    UPDATED_CONTENT_DIRS
        .iter()
        .flat_map(|dir| paths::expand(dir, home.as_deref()))
        .next()
}

fn get(url: &str, wait: Duration) -> Result<Vec<u8>, String> {
    internet::download(url, UPDATE_MAX_BYTES, wait)
}

/// The weekly bundle: zstd, then records. An empty one is a failure, so
/// tldr is tried.
pub fn from_bundle(packed: &[u8]) -> Result<Vec<NewPage>, String> {
    let mut text = String::new();
    ruzstd::decoding::StreamingDecoder::new(packed)
        .map_err(|e| e.to_string())?
        .take(UPDATE_MAX_BYTES as u64)
        .read_to_string(&mut text)
        .map_err(|e| e.to_string())?;
    let pages = records(&text);
    if pages.is_empty() {
        return Err(ERROR_UPDATE_EMPTY.to_owned());
    }
    Ok(pages)
}

/// The pages in a bundle's text: each starts with `RECORD_MARK`, its path
/// (`tar.md`, `i18n/pt_BR/tar.md`) and a newline. A record with any other
/// path is dropped.
pub fn records(text: &str) -> Vec<NewPage> {
    text.split(RECORD_MARK)
        .filter_map(|record| {
            let (path, page) = record.split_once('\n')?;
            let (lang, name) = page_path(path.trim())?;
            Some(NewPage {
                lang,
                name,
                text: page.to_owned(),
            })
        })
        .collect()
}

/// `tar.md` → (None, tar); `i18n/pt_BR/tar.md` → (pt_BR, tar). Anything
/// that could leave the folder (`..`, a slash in the name, a dot first) is
/// refused.
pub fn page_path(path: &str) -> Option<(Option<String>, String)> {
    let parts: Vec<&str> = path.split('/').collect();
    let (lang, file) = match parts.as_slice() {
        [file] => (None, *file),
        ["i18n", lang, file] => (Some(*lang), *file),
        _ => return None,
    };
    let name = file.strip_suffix(".md")?;
    let good_lang = lang.is_none_or(|l| {
        !l.is_empty()
            && l.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    });
    if name.is_empty() || name.starts_with('.') || name.contains('\\') || !good_lang {
        return None;
    }
    Some((lang.map(str::to_owned), name.to_owned()))
}

/// tldr-pages' English zip and the reader's language's, converted.
fn from_tldr(say: &mut dyn FnMut(&str), wait: Duration) -> Result<Vec<NewPage>, String> {
    let platforms = if cfg!(target_os = "macos") {
        TLDR_PLATFORMS_MACOS
    } else {
        TLDR_PLATFORMS_LINUX
    };
    let url = UPDATE_TLDR_URL.replace("{lang}", "en");
    say(&UPDATE_TRYING.replace("{url}", &url));
    let english = get(&url, wait).map_err(|why| ERROR_UPDATE_FAILED.replace("{why}", &why))?;
    let mut pages = from_tldr_zip(&english, platforms, None);
    // the first of the reader's languages tldr has (pt_BR, else pt)
    for lang in content::languages() {
        let url = UPDATE_TLDR_URL.replace("{lang}", &lang);
        say(&UPDATE_TRYING.replace("{url}", &url));
        if let Ok(zip) = get(&url, wait) {
            pages.extend(from_tldr_zip(&zip, platforms, Some(&lang)));
            break;
        }
    }
    Ok(pages)
}

/// The pages in one of tldr's zips (`linux/apt.md`…), from `platforms`
/// only, the first platform winning a name; `lang`: a translation's.
pub fn from_tldr_zip(zip_bytes: &[u8], platforms: &[&str], lang: Option<&str>) -> Vec<NewPage> {
    let files = zip::files(zip_bytes, PAGE_MAX_BYTES);
    let mut pages: Vec<NewPage> = Vec::new();
    for platform in platforms {
        for (path, bytes) in &files {
            let Some(name) = path
                .strip_prefix(platform)
                .and_then(|p| p.strip_prefix('/'))
                .and_then(|p| p.strip_suffix(".md"))
            else {
                continue;
            };
            if name.contains('/') || pages.iter().any(|p| p.name == name) {
                continue;
            }
            let text = String::from_utf8_lossy(bytes);
            if let Some(text) = tldr::convert(name, platform, &text, lang.is_none()) {
                pages.push(NewPage {
                    lang: lang.map(str::to_owned),
                    name: name.to_owned(),
                    text,
                });
            }
        }
    }
    pages
}

/// Whether lore needs `page`: no built-in page (or translation) of that
/// name, and it passes the check.
pub fn wanted(page: &NewPage) -> bool {
    let missing = match &page.lang {
        None => content::built_in(&page.name).is_none(),
        Some(lang) => content::translation(lang, &page.name).is_none(),
    };
    missing && check::passes(&check::check(&page.name, page.text.as_bytes()))
}

/// Writes the wanted pages into `dir`, replacing what was there: written
/// beside it first, then swapped, so a failure leaves the old pages.
/// Returns (pages, translations) written.
pub fn install(dir: &Path, pages: Vec<NewPage>) -> io::Result<(usize, usize)> {
    let fresh = dir.with_extension("new");
    let _ = fs::remove_dir_all(&fresh);
    fs::create_dir_all(&fresh)?;
    let (mut new_pages, mut translations) = (0, 0);
    for page in pages.into_iter().filter(wanted) {
        let folder = match &page.lang {
            None => fresh.clone(),
            Some(lang) => fresh.join("i18n").join(lang),
        };
        fs::create_dir_all(&folder)?;
        let file = folder.join(format!("{}.md", page.name));
        if file.exists() {
            continue; // the first of a name wins
        }
        fs::write(file, page.text)?;
        match page.lang {
            None => new_pages += 1,
            Some(_) => translations += 1,
        }
    }
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }
    fs::rename(&fresh, dir)?;
    Ok((new_pages, translations))
}
