//! Colour palettes: the colours lore draws with. Each one is a file,
//! `palettes/<name>.palette` in the repository (built into lore by
//! build.rs, see palettes/README.md), or one of your own in
//! `~/.config/lore/palettes/` (`$XDG_CONFIG_HOME/lore/palettes/`), read at
//! start; yours win over a built-in one of the same name. Picked in the app
//! (`p`) and saved in the settings file, or named for one run by
//! `$LORE_PALETTE`.
//!
//! The first palette, `terminal`, uses the terminal's own colours. The others
//! paint their own background and text, so a light palette reads the same on
//! a dark terminal and the other way round.

use std::fs;
use std::path::Path;

use ratatui::style::{Color, Modifier, Style};

use crate::configs::{
    COLOUR_NAMES, MODIFIER_DANGER, MODIFIER_HEADING, MODIFIER_SECTION, PALETTE_DIR,
    PALETTE_FILE_END, PALETTE_VAR, SETTING_PALETTE,
};
use crate::settings;

// BUILT_IN_PALETTES: (name, text) of every palettes/*.palette, `terminal` first
include!(concat!(env!("OUT_DIR"), "/palettes.rs"));

/// One palette. `Color::Reset` leaves a colour to the terminal: for `muted`
/// the text is dimmed instead, for `selection` the row is reversed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palette {
    /// The file's name without `.palette`; what `$LORE_PALETTE` names it by.
    pub name: String,
    pub background: Color,
    pub text: Color,
    /// Hints, what an alias runs, the manual's quieter lines.
    pub muted: Color,
    /// The selected row's background.
    pub selection: Color,
    /// The theme's main colour: letter headers, page headings, the border of
    /// the search bar while typing and of the palette picker.
    pub primary: Color,
    /// Its second colour: the categories.
    pub secondary: Color,
    /// Search snippets and links.
    pub accent: Color,
    pub danger: Color,
    /// Usage examples.
    pub code: Color,
    /// Warnings.
    pub warning: Color,
}

/// A palette file's text, read: `key = value` lines, `#` comments, every
/// colour `#rrggbb` or a name from `COLOUR_NAMES`. The error names the key
/// that is missing or wrong.
pub fn parse(name: &str, text: &str) -> Result<Palette, String> {
    let colour = |key: &str| {
        let value = settings::get(text, key).ok_or_else(|| format!("`{key}` is missing"))?;
        colour(&value).ok_or_else(|| format!("`{key} = {value}` is not a colour"))
    };
    Ok(Palette {
        name: name.to_owned(),
        background: colour("background")?,
        text: colour("text")?,
        muted: colour("muted")?,
        selection: colour("selection")?,
        primary: colour("primary")?,
        secondary: colour("secondary")?,
        accent: colour("accent")?,
        danger: colour("danger")?,
        code: colour("code")?,
        warning: colour("warning")?,
    })
}

/// `#2e3440` or `cyan`, ignoring case.
pub fn colour(value: &str) -> Option<Color> {
    let value = value.trim().to_lowercase();
    if let Some(hex) = value.strip_prefix('#') {
        if hex.len() != 6 {
            return None;
        }
        let part = |i: usize| u8::from_str_radix(hex.get(i..i + 2)?, 16).ok();
        return Some(Color::Rgb(part(0)?, part(2)?, part(4)?));
    }
    COLOUR_NAMES
        .iter()
        .find(|(name, _)| *name == value)
        .map(|(_, colour)| *colour)
}

/// The palettes built into lore, `terminal` first. build.rs checked them,
/// so none fails here (a test checks it too).
pub fn built_in() -> Vec<Palette> {
    BUILT_IN_PALETTES
        .iter()
        .filter_map(|(name, text)| parse(name, text).ok())
        .collect()
}

/// The built-in palettes and yours from the settings folder.
pub fn all() -> Vec<Palette> {
    let mut palettes = built_in();
    if let Some(dir) = settings::path().as_deref().and_then(Path::parent) {
        add_from(&mut palettes, &dir.join(PALETTE_DIR));
    }
    palettes
}

/// Adds the `*.palette` files in `dir` to `palettes`, by name; one with a
/// built-in palette's name takes its place. A missing folder or a file that
/// does not parse is skipped: lore opens anyway, in the palettes it has.
pub fn add_from(palettes: &mut Vec<Palette>, dir: &Path) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut files: Vec<_> = entries.flatten().map(|e| e.path()).collect();
    files.sort();
    for path in files {
        let Some(name) = path
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(|n| n.strip_suffix(PALETTE_FILE_END))
        else {
            continue;
        };
        let Some(palette) = fs::read_to_string(&path)
            .ok()
            .and_then(|t| parse(name, &t).ok())
        else {
            continue;
        };
        match palettes.iter_mut().find(|p| p.name == palette.name) {
            Some(same) => *same = palette,
            None => palettes.push(palette),
        }
    }
}

/// Where in `palettes` to start: `$LORE_PALETTE` when set (for one run),
/// else the one saved in the settings file, else `terminal`. An unknown name
/// gives `terminal` too: a typo should not stop lore from opening.
pub fn at_start(palettes: &[Palette]) -> usize {
    let for_this_run = std::env::var(PALETTE_VAR)
        .ok()
        .filter(|name| !name.trim().is_empty());
    named(
        palettes,
        &for_this_run
            .or_else(|| settings::read(SETTING_PALETTE))
            .unwrap_or_default(),
    )
}

/// Where the palette called `name` is in `palettes`, ignoring case and `_`
/// for `-`; 0 (`terminal`) when there is none by that name.
pub fn named(palettes: &[Palette], name: &str) -> usize {
    let wanted = name.trim().to_lowercase().replace('_', "-");
    palettes.iter().position(|p| p.name == wanted).unwrap_or(0)
}

impl Palette {
    /// The whole screen: background and plain text.
    pub fn base(&self) -> Style {
        Style::new().fg(self.text).bg(self.background)
    }
    pub fn muted(&self) -> Style {
        if self.muted == Color::Reset {
            Style::new().add_modifier(Modifier::DIM)
        } else {
            Style::new().fg(self.muted)
        }
    }
    pub fn selected(&self) -> Style {
        if self.selection == Color::Reset {
            Style::new().add_modifier(Modifier::REVERSED)
        } else {
            Style::new().bg(self.selection)
        }
    }
    /// Each key on the key bar.
    pub fn key(&self) -> Style {
        Style::new().add_modifier(Modifier::REVERSED)
    }
    pub fn heading(&self) -> Style {
        Style::new().fg(self.primary).add_modifier(MODIFIER_HEADING)
    }
    pub fn category(&self) -> Style {
        Style::new().fg(self.secondary)
    }
    /// Borders that show where the keys go: the search bar while typing,
    /// the palette picker.
    pub fn active(&self) -> Style {
        Style::new().fg(self.primary)
    }
    pub fn accent(&self) -> Style {
        Style::new().fg(self.accent)
    }
    pub fn section(&self) -> Style {
        Style::new().fg(self.primary).add_modifier(MODIFIER_SECTION)
    }
    pub fn danger(&self) -> Style {
        Style::new().fg(self.danger).add_modifier(MODIFIER_DANGER)
    }
    pub fn code(&self) -> Style {
        Style::new().fg(self.code)
    }
    pub fn warning(&self) -> Style {
        Style::new().fg(self.warning)
    }
}
