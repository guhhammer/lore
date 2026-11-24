//! Hardcoded values: the beacon topic (src/info/beacon.rs).

/// The numbers built into lore: beacon/beacon.tsv, from
/// scripts/beacon/snapshot.py (popcon and Homebrew).
pub const BEACON_SNAPSHOT: &str = include_str!("../../beacon/beacon.tsv");
/// A newer copy, fetched in the background when Internet is on, and kept
/// in `$XDG_CACHE_HOME/lore/beacon.tsv` for `BEACON_CACHE_HOURS`.
pub const BEACON_URL: &str =
    "https://github.com/guhhammer/lore/releases/download/content/beacon.tsv";
pub const BEACON_CACHE_FILE: &str = "lore/beacon.tsv";
pub const BEACON_CACHE_HOURS: u64 = 24 * 7;
pub const BEACON_MAX_BYTES: usize = 4 * 1024 * 1024;
/// Without lore's copy, the sources themselves: popcon's ranking (read only
/// this far: the most installed come first) and Homebrew's installs asked
/// for by name. A package or formula then counts for the command of the
/// same name only.
pub const BEACON_POPCON_URL: &str = "https://popcon.debian.org/by_inst";
pub const BEACON_POPCON_MAX_BYTES: usize = 3 * 1024 * 1024;
pub const BEACON_BREW_URL: &str =
    "https://formulae.brew.sh/api/analytics/install-on-request/{days}d.json";
/// A package on this share of the most installed one's machines is the base
/// system, not a choice: left out (scripts/beacon/snapshot.py does the same).
pub const BEACON_BASE_SHARE: f64 = 0.9;
/// How many commands a table made here keeps, the most used first (as the
/// script's).
pub const BEACON_KEEP: usize = 1500;
/// Trending: installs in the last 30 days against the year's monthly mean,
/// at least this much higher, from at least `BEACON_TREND_MIN` installs.
pub const BEACON_TREND_MIN_RISE: f64 = 1.3;
pub const BEACON_TREND_MIN: u64 = 2000;
/// How many the beacon lists: trending, popular, and per category.
pub const BEACON_TRENDING_MAX: usize = 20;
pub const BEACON_POPULAR_MAX: usize = 50;
pub const BEACON_PER_CATEGORY: usize = 5;
pub const BEACON_CATEGORIES_MAX: usize = 12;
/// How many of the most popular the categories are made from.
pub const BEACON_CATEGORY_POOL: usize = 400;
/// The beacon's headings, and what follows each name.
pub const BEACON_TRENDING: &str = "trending this month";
pub const BEACON_POPULAR: &str = "most used";
pub const BEACON_FOR: &str = "most used for {category}";
pub const BEACON_RISE: &str = "↑ {rise}× this month";
pub const BEACON_RANK: &str = "#{rank}";
pub const BEACON_NOT_HERE: &str = "not installed";
