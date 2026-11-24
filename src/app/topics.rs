//! The topics: what each one lists, the beacon's and the docs' rows, and
//! your own lists (bookmarks, recent, most used).

use std::collections::HashMap;

use crate::commands::{Command, Kind};
use crate::configs::{
    BEACON_CATEGORIES_MAX, BEACON_CATEGORY_POOL, BEACON_FOR, BEACON_NOT_HERE, BEACON_PER_CATEGORY,
    BEACON_POPULAR, BEACON_POPULAR_MAX, BEACON_RANK, BEACON_RISE, BEACON_TRENDING,
    BEACON_TRENDING_MAX, DANGER, DOCS_ALL, DOCS_DOWNLOADED, DOCS_DOWNLOADING, DOCS_FAILED,
    DOCS_HERE, DOCS_MOST, DOCS_MOST_SEARCHED, DOCS_SIZE, MOST_USED_MAX,
};
use crate::info::content;
use crate::info::{beacon, docs};
use crate::interface::rows::Row;
use crate::interface::search;

use super::{App, Mode, Topic, docs_of};

impl App {
    /// Your bookmarks, recent pages and history at start (lib.rs reads them).
    pub fn set_personal(
        &mut self,
        bookmarks: Vec<String>,
        recent: Vec<String>,
        uses: HashMap<String, u32>,
    ) {
        self.bookmarks = bookmarks;
        self.recent = recent;
        self.uses = uses;
        self.refilter();
    }

    /// Bookmarks the command in view (the page's, or the selected one), or
    /// takes its bookmark away. False when there is no command in view.
    pub fn toggle_bookmark(&mut self) -> bool {
        let in_view = match (self.mode, &self.page) {
            (Mode::Info, Some(page)) => Some(page.command.name.clone()),
            _ => self.selected().map(|c| c.name.clone()),
        };
        let Some(name) = in_view else {
            return false;
        };
        if self.is_bookmarked(&name) {
            self.bookmarks.retain(|n| *n != name);
        } else {
            self.bookmarks.push(name);
        }
        self.rebuild_keeping_selection();
        true
    }

    pub fn is_bookmarked(&self, name: &str) -> bool {
        self.bookmarks.iter().any(|n| n == name)
    }

    /// A command on this machine your shell history never ran; never true
    /// without a history, nor for one not installed (the beacon's), which
    /// says so itself.
    pub fn is_unused(&self, name: &str) -> bool {
        !self.uses.is_empty()
            && !self.uses.contains_key(name)
            && self.commands.iter().any(|c| c.name == name)
    }

    /// The commands of `topic`, among `allowed` (on this machine, `@`
    /// respected), in the topic's own order.
    pub(super) fn topic_list(&self, topic: Topic, allowed: &[&Command]) -> Vec<Command> {
        let named = |names: &[String]| -> Vec<Command> {
            names
                .iter()
                .filter_map(|name| allowed.iter().find(|c| c.name == *name))
                .map(|c| (*c).clone())
                .collect()
        };
        match topic {
            Topic::Commands => allowed
                .iter()
                .filter(|c| !matches!(c.kind, Kind::Recipe | Kind::Tool | Kind::Language))
                .map(|c| (*c).clone())
                .collect(),
            Topic::Languages => allowed
                .iter()
                .filter(|c| c.kind == Kind::Language)
                .map(|c| (*c).clone())
                .collect(),
            Topic::Recipes => allowed
                .iter()
                .filter(|c| c.kind == Kind::Recipe)
                .map(|c| (*c).clone())
                .collect(),
            Topic::MostUsed => {
                let mut used: Vec<&&Command> = allowed
                    .iter()
                    .filter(|c| self.uses.contains_key(&c.name))
                    .collect();
                used.sort_by(|a, b| {
                    self.uses[&b.name]
                        .cmp(&self.uses[&a.name])
                        .then_with(|| a.name.cmp(&b.name))
                });
                used.into_iter()
                    .take(MOST_USED_MAX)
                    .map(|c| (*c).clone())
                    .collect()
            }
            Topic::Tools => allowed
                .iter()
                .filter(|c| c.kind == Kind::Tool)
                .map(|c| (*c).clone())
                .collect(),
            Topic::Recent => named(&self.recent),
            Topic::Bookmarks => named(&self.bookmarks),
            // headed rows of their own: `beacon_rows`, `docs_rows`
            Topic::Beacon | Topic::Docs => Vec::new(),
        }
    }

    /// Whether `topic` has anything among `allowed`, without building its
    /// list; commands always counts, so the bar is never empty.
    pub(super) fn has_any(&self, topic: Topic, allowed: &[&Command]) -> bool {
        let on_machine = |name: &String| allowed.iter().any(|c| c.name == *name);
        match topic {
            Topic::Commands => true,
            Topic::Recipes => allowed.iter().any(|c| c.kind == Kind::Recipe),
            Topic::Languages => allowed.iter().any(|c| c.kind == Kind::Language),
            Topic::Beacon => !self.beacon.is_empty(),
            Topic::Docs | Topic::Tools => true,
            Topic::MostUsed => allowed.iter().any(|c| self.uses.contains_key(&c.name)),
            Topic::Recent => self.recent.iter().any(on_machine),
            Topic::Bookmarks => self.bookmarks.iter().any(on_machine),
        }
    }

    /// Sets the beacon's numbers, and reads the categories of the most used
    /// among them from their pages, once.
    pub fn set_beacon(&mut self, entries: Vec<beacon::Entry>) {
        self.beacon_categories = beacon::popular(&entries)
            .into_iter()
            .take(BEACON_CATEGORY_POOL)
            .filter_map(|e| Some((e.name.clone(), content::find(&e.name)?.categories)))
            .collect();
        self.beacon = entries;
        self.refilter();
    }

    /// The beacon's rows, under their headings, and what follows each name
    /// (its rank, its rise, "not installed"). A command not on this machine
    /// is listed when lore has a page for it, to read about before
    /// installing it.
    pub(super) fn beacon_rows(&self, allowed: &[&Command]) -> (Vec<Row>, HashMap<String, String>) {
        let listed = |name: &str| -> Option<Command> {
            match allowed.iter().find(|c| c.name == name) {
                Some(command) => Some((*command).clone()),
                None => self
                    .pages
                    .contains(&content::page_key(name))
                    .then(|| Command::new(name, Kind::Program)),
            }
        };
        let popular: Vec<&beacon::Entry> = beacon::popular(&self.beacon)
            .into_iter()
            .filter(|e| listed(&e.name).is_some())
            .collect();
        let mut snippets = HashMap::new();
        for (rank, entry) in popular.iter().enumerate() {
            let mut parts = vec![BEACON_RANK.replace("{rank}", &(rank + 1).to_string())];
            if let Some(rise) = beacon::rise(entry) {
                parts.push(BEACON_RISE.replace("{rise}", &format!("{rise:.1}")));
            }
            if !allowed.iter().any(|c| c.name == entry.name) {
                parts.push(BEACON_NOT_HERE.to_owned());
            }
            snippets.insert(entry.name.clone(), parts.join("  "));
        }
        let mut rows = Vec::new();
        let mut section = |title: String, names: Vec<&str>| {
            let commands: Vec<Command> = names.into_iter().filter_map(listed).collect();
            if !commands.is_empty() {
                rows.push(Row::Title(title));
                rows.extend(commands.into_iter().map(Row::Command));
            }
        };
        let trending = beacon::trending(&self.beacon)
            .into_iter()
            .filter(|(e, _)| listed(&e.name).is_some())
            .take(BEACON_TRENDING_MAX)
            .map(|(e, _)| e.name.as_str())
            .collect();
        section(BEACON_TRENDING.to_owned(), trending);
        let most = popular
            .iter()
            .take(BEACON_POPULAR_MAX)
            .map(|e| e.name.as_str());
        section(BEACON_POPULAR.to_owned(), most.collect());
        // per category, in the order their most used command comes
        let mut groups: Vec<(&str, Vec<&str>)> = Vec::new();
        for entry in &popular {
            let categories = self.beacon_categories.get(&entry.name);
            for category in categories.into_iter().flatten().filter(|c| *c != DANGER) {
                match groups.iter_mut().find(|(c, _)| c == category) {
                    Some((_, names)) if names.len() < BEACON_PER_CATEGORY => {
                        names.push(&entry.name);
                    }
                    Some(_) => {}
                    None => groups.push((category, vec![&entry.name])),
                }
            }
        }
        for (category, names) in groups
            .into_iter()
            .filter(|(_, names)| names.len() == BEACON_PER_CATEGORY)
            .take(BEACON_CATEGORIES_MAX)
        {
            section(BEACON_FOR.replace("{category}", category), names);
        }
        (rows, snippets)
    }

    /// Shows `topic`, if it is in the bar, its list from the top.
    pub fn show_topic(&mut self, topic: Topic) {
        if self.topics.contains(&topic) {
            self.topic = topic;
            self.refilter();
        }
    }

    /// Shows the topic `steps` away in the bar, wrapping round, its list
    /// from the top.
    pub fn step_topic(&mut self, steps: isize) {
        let count = self.topics.len().max(1);
        let at = self
            .topics
            .iter()
            .position(|t| *t == self.topic)
            .unwrap_or(0);
        let forward = steps.rem_euclid(isize::try_from(count).unwrap_or(isize::MAX));
        let next = (at + usize::try_from(forward).unwrap_or(0)) % count;
        self.topic = self.topics.get(next).copied().unwrap_or(Topic::Commands);
        self.refilter();
    }

    /// Leaves the set whose pages are listed, back to the sets; false when
    /// none is open.
    pub fn close_docset(&mut self) -> bool {
        if self.docset.take().is_none() {
            return false;
        }
        self.query.clear();
        self.refilter();
        true
    }

    /// The docs topic's rows: an open set's pages under their types, else
    /// the sets: downloaded, most searched, then every one, each with its
    /// state. With a `query`, only those whose name matches, best first.
    pub(super) fn docs_rows(&self, query: &str) -> (Vec<Row>, HashMap<String, String>) {
        let mut rows = Vec::new();
        let mut snippets = HashMap::new();
        if let Some((set, entries)) = &self.docset {
            let entry = |e: &docs::Entry| Command {
                runs: Some(format!("{}\t{}\t{}", set.slug, e.path, e.kind)),
                ..Command::new(e.name.clone(), Kind::DocEntry)
            };
            if query.trim().is_empty() {
                rows.push(Row::Title(set.title()));
                let mut kinds: Vec<&str> = entries.iter().map(|e| e.kind.as_str()).collect();
                kinds.sort_unstable();
                kinds.dedup();
                for kind in kinds {
                    rows.push(Row::Title(kind.to_owned()));
                    let mut these: Vec<&docs::Entry> =
                        entries.iter().filter(|e| e.kind == kind).collect();
                    these.sort_by_key(|e| e.name.to_lowercase());
                    rows.extend(these.into_iter().map(|e| Row::Command(entry(e))));
                }
            } else {
                let mut found: Vec<(_, &docs::Entry)> = entries
                    .iter()
                    .filter_map(|e| Some((search::name_rank(&e.name, query)?, e)))
                    .collect();
                found.sort_by(|(a, x), (b, y)| a.cmp(b).then_with(|| x.name.cmp(&y.name)));
                rows.extend(found.into_iter().map(|(_, e)| Row::Command(entry(e))));
            }
            return (rows, snippets);
        }
        let row = |set: &docs::Docset| Command {
            runs: Some(set.slug.clone()),
            ..Command::new(set.title(), Kind::Docset)
        };
        for set in &self.docsets {
            let state = match self.downloads.progress(&set.slug) {
                Some(docs::Progress::Downloading) => DOCS_DOWNLOADING.to_owned(),
                Some(docs::Progress::Failed(why)) => DOCS_FAILED.replace("{why}", &why),
                None if docs::is_downloaded(&set.slug) => DOCS_HERE.to_owned(),
                None => {
                    DOCS_SIZE.replace("{size}", &format!("{:.1}", set.bytes as f64 / 1_048_576.0))
                }
            };
            snippets.insert(set.title(), state);
        }
        if !query.trim().is_empty() {
            let mut found: Vec<(_, &docs::Docset)> = self
                .docsets
                .iter()
                .filter_map(|s| Some((search::name_rank(&s.title(), query)?, s)))
                .collect();
            found.sort_by(|(a, x), (b, y)| a.cmp(b).then_with(|| x.rank.cmp(&y.rank)));
            rows.extend(found.into_iter().map(|(_, s)| Row::Command(row(s))));
            return (rows, snippets);
        }
        let here: Vec<&docs::Docset> = self
            .docsets
            .iter()
            .filter(|s| docs::is_downloaded(&s.slug))
            .collect();
        let mut section = |title: &str, sets: Vec<&docs::Docset>| {
            if !sets.is_empty() {
                rows.push(Row::Title(title.to_owned()));
                rows.extend(sets.into_iter().map(|s| Row::Command(row(s))));
            }
        };
        section(DOCS_DOWNLOADED, here);
        section(
            DOCS_MOST,
            docs::most_searched(&self.docsets, DOCS_MOST_SEARCHED),
        );
        let mut all: Vec<&docs::Docset> = self.docsets.iter().collect();
        all.sort_by_key(|s| s.title().to_lowercase());
        section(DOCS_ALL, all);
        (rows, snippets)
    }

    /// Reads the pages again for the search, in the language pages are
    /// read in now (Pages in changed), and searches again.
    pub fn reload_docs(&mut self) {
        self.docs = docs_of(&self.commands, &self.pages);
        self.refilter();
    }
}
