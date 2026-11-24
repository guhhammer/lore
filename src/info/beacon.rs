//! The beacon: what people use most, and what is rising, from two public
//! sources: Debian's popularity contest (machines that use a package
//! regularly) and Homebrew's analytics (installs asked for by name, over a
//! year and over the last 30 days). No one's own use is sent anywhere: lore
//! only reads these published numbers.
//!
//! The numbers are a small table, one command a line (beacon/beacon.tsv,
//! built in; scripts/beacon/snapshot.py makes it):
//!
//! ```text
//! # comments
//! git<TAB>132077<TAB>1374697<TAB>135180
//! ```
//!
//! (command, popcon votes, Homebrew installs in 365 days, in 30 days). With
//! Internet on, a newer table is fetched in the background at most once a
//! `BEACON_CACHE_HOURS`, first from lore's repository (`BEACON_URL`, made
//! each week), else from the sources themselves; the next start uses it.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use crate::compile_config::{
    BEACON_BASE_SHARE, BEACON_BREW_URL, BEACON_CACHE_FILE, BEACON_CACHE_HOURS, BEACON_KEEP,
    BEACON_MAX_BYTES, BEACON_POPCON_MAX_BYTES, BEACON_POPCON_URL, BEACON_SNAPSHOT,
    BEACON_TREND_MIN, BEACON_TREND_MIN_RISE, BEACON_URL, INTERNET_TIMEOUT,
};
use crate::info::content;
use crate::utils::internet;

/// One command's numbers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    /// Debian machines that use it regularly (popcon's "vote").
    pub debian: u64,
    /// Homebrew installs asked for by name, in the last 365 and 30 days.
    pub brew_year: u64,
    pub brew_month: u64,
}

/// The table's entries; lines that are not `name<TAB>n<TAB>n<TAB>n` are
/// skipped.
pub fn parse(text: &str) -> Vec<Entry> {
    text.lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| {
            let mut fields = line.split('\t');
            let name = fields.next()?.trim();
            let mut number = || fields.next()?.trim().parse::<u64>().ok();
            let entry = Entry {
                name: name.to_owned(),
                debian: number()?,
                brew_year: number()?,
                brew_month: number()?,
            };
            let plain = !name.is_empty()
                && !name.starts_with('.')
                && name
                    .chars()
                    .all(|c| c.is_alphanumeric() || "._+-".contains(c));
            plain.then_some(entry)
        })
        .collect()
}

/// The table, as `parse` reads it.
pub fn to_tsv(entries: &[Entry]) -> String {
    entries
        .iter()
        .map(|e| {
            format!(
                "{}\t{}\t{}\t{}\n",
                e.name, e.debian, e.brew_year, e.brew_month
            )
        })
        .collect()
}

/// Most used first: each source's share of the table, added, so Debian
/// and Homebrew count alike.
pub fn popular(entries: &[Entry]) -> Vec<&Entry> {
    let total = |f: fn(&Entry) -> u64| entries.iter().map(f).sum::<u64>().max(1) as f64;
    let (debian, year) = (total(|e| e.debian), total(|e| e.brew_year));
    let score = |e: &Entry| e.debian as f64 / debian + e.brew_year as f64 / year;
    let mut sorted: Vec<&Entry> = entries.iter().collect();
    sorted.sort_by(|a, b| {
        score(b)
            .total_cmp(&score(a))
            .then_with(|| a.name.cmp(&b.name))
    });
    sorted
}

/// How much more it was installed in the last 30 days than in a mean month
/// of the year (2.0: twice as much); None below the thresholds.
pub fn rise(entry: &Entry) -> Option<f64> {
    if entry.brew_month < BEACON_TREND_MIN || entry.brew_year == 0 {
        return None;
    }
    let rise = entry.brew_month as f64 * 12.0 / entry.brew_year as f64;
    (rise >= BEACON_TREND_MIN_RISE).then_some(rise)
}

/// The rising ones, the most risen first.
pub fn trending(entries: &[Entry]) -> Vec<(&Entry, f64)> {
    let mut rising: Vec<(&Entry, f64)> =
        entries.iter().filter_map(|e| Some((e, rise(e)?))).collect();
    rising.sort_by(|(a, x), (b, y)| y.total_cmp(x).then_with(|| a.name.cmp(&b.name)));
    rising
}

/// The newest table there is: the one fetched last, else the built-in one.
pub fn load() -> Vec<Entry> {
    cache_file()
        .and_then(|path| fs::read_to_string(path).ok())
        .map(|text| parse(&text))
        .filter(|entries| !entries.is_empty())
        .unwrap_or_else(|| parse(BEACON_SNAPSHOT))
}

/// `$XDG_CACHE_HOME/lore/beacon.tsv`, or `~/.cache/lore/beacon.tsv`.
fn cache_file() -> Option<PathBuf> {
    Some(internet::cache_home()?.join(BEACON_CACHE_FILE))
}

/// Fetches a newer table in the background when the kept one is missing or
/// older than `BEACON_CACHE_HOURS`, for the next start. Only called with
/// Internet on.
pub fn refresh() {
    let Some(path) = cache_file() else {
        return;
    };
    let fresh = internet::age(&path)
        .is_some_and(|age| age < Duration::from_secs(BEACON_CACHE_HOURS * 3600));
    if fresh {
        return;
    }
    thread::spawn(move || {
        let wait = INTERNET_TIMEOUT * 10;
        let fetched = internet::download(BEACON_URL, BEACON_MAX_BYTES, wait)
            .ok()
            .map(|bytes| parse(&String::from_utf8_lossy(&bytes)))
            .filter(|entries| !entries.is_empty())
            .or_else(|| from_sources(wait));
        if let (Some(entries), Some(dir)) = (fetched, path.parent()) {
            let _ = fs::create_dir_all(dir);
            let _ = fs::write(&path, to_tsv(&entries));
        }
    });
}

/// The table made here from the sources, when lore's copy cannot be had.
fn from_sources(wait: Duration) -> Option<Vec<Entry>> {
    let get = |url: &str, max| internet::download(url, max, wait).ok();
    let popcon = get(BEACON_POPCON_URL, BEACON_POPCON_MAX_BYTES)?;
    let brew = |days: &str| get(&BEACON_BREW_URL.replace("{days}", days), BEACON_MAX_BYTES);
    let (year, month) = (brew("365")?, brew("30")?);
    let entries = combine(
        &String::from_utf8_lossy(&popcon),
        &String::from_utf8_lossy(&year),
        &String::from_utf8_lossy(&month),
        &|name| content::built_in(name).is_some(),
    );
    (!entries.is_empty()).then_some(entries)
}

/// The table from the sources' texts: popcon's `by_inst`, Homebrew's
/// 365-day and 30-day JSON. Here a package or formula counts for the
/// command of the same name only, and only commands `has_page` keeps;
/// the most used `BEACON_KEEP`.
pub fn combine(
    popcon: &str,
    year: &str,
    month: &str,
    has_page: &dyn Fn(&str) -> bool,
) -> Vec<Entry> {
    let debian = popcon_votes(popcon);
    let (year, month) = (brew_counts(year), brew_counts(month));
    let mut names: Vec<&String> = debian.keys().chain(year.keys()).collect();
    names.sort();
    names.dedup();
    let entries: Vec<Entry> = names
        .into_iter()
        .filter(|name| has_page(name))
        .map(|name| Entry {
            name: name.clone(),
            debian: debian.get(name).copied().unwrap_or(0),
            brew_year: year.get(name).copied().unwrap_or(0),
            brew_month: month.get(name).copied().unwrap_or(0),
        })
        .collect();
    popular(&entries)
        .into_iter()
        .take(BEACON_KEEP)
        .cloned()
        .collect()
}

/// popcon's `by_inst`: `rank name inst vote old recent no-files (who)`,
/// the most installed first. Package → votes, the base system left out
/// (installed on nearly every machine: nobody chose it).
pub fn popcon_votes(text: &str) -> HashMap<String, u64> {
    let mut votes = HashMap::new();
    let mut top = 0;
    for line in text.lines().filter(|l| !l.starts_with('#')) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let (Some(name), Some(Ok(inst)), Some(Ok(vote))) = (
            fields.get(1),
            fields.get(2).map(|f| f.parse::<u64>()),
            fields.get(3).map(|f| f.parse::<u64>()),
        ) else {
            continue;
        };
        top = top.max(inst);
        if (inst as f64) < top as f64 * BEACON_BASE_SHARE {
            votes.insert((*name).to_owned(), vote);
        }
    }
    votes
}

/// Homebrew's analytics JSON: each item's `"formula": "name"` and
/// `"count": "1,234"`. Formula → installs, a version suffix dropped
/// (`python@3.13` counts for `python`). Read by hand: two fields of a
/// known file are not worth a JSON library.
pub fn brew_counts(json: &str) -> HashMap<String, u64> {
    let mut counts: HashMap<String, u64> = HashMap::new();
    for item in json.split('{').skip(1) {
        let (Some(formula), Some(count)) = (field(item, "formula"), field(item, "count")) else {
            continue;
        };
        let Ok(count) = count.replace(',', "").parse::<u64>() else {
            continue;
        };
        let name = formula.split('@').next().unwrap_or_default();
        *counts.entry(name.to_owned()).or_default() += count;
    }
    counts
}

/// The string value of `"key":"value"` in one JSON object's text.
fn field<'a>(item: &'a str, key: &str) -> Option<&'a str> {
    let after = &item[item.find(&format!("\"{key}\""))? + key.len() + 2..];
    let after = after.trim_start().strip_prefix(':')?.trim_start();
    let value = after.strip_prefix('"')?;
    Some(&value[..value.find('"')?])
}
