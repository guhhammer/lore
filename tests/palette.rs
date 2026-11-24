//! Tests for the colour palettes (src/interface/palette.rs, palettes/).

use std::collections::{HashMap, HashSet};
use std::fs;

use lore_tui::app::{App, Mode};
use lore_tui::commands::{Command, Kind};
use lore_tui::configs::PALETTE_KEY;
use lore_tui::interface::input::{Action, action_for};
use lore_tui::interface::palette::{Palette, add_from, built_in, colour, named, parse};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::{Color, Modifier};

/// The built-in palette called `name`.
fn palette(name: &str) -> Palette {
    let palettes = built_in();
    palettes[named(&palettes, name)].clone()
}

/// A scratch folder for palette files, empty, in the system's temp dir.
fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("lore-test-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

const NORD_LIKE: &str = "# mine\nbackground = #000000\ntext = #FFFFFF\nmuted = gray\n\
selection = #111111\nprimary = cyan\nsecondary = blue\naccent = #00ff00\ndanger = red\n\
code = green\nwarning = yellow\n";

#[test]
fn every_file_in_palettes_parses_and_the_default_comes_first() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/palettes");
    let files = fs::read_dir(dir)
        .unwrap()
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "palette"))
        .count();
    let palettes = built_in();
    assert_eq!(palettes.len(), files, "a palette file did not parse");
    assert_eq!(palettes[0].name, "terminal");
}

#[test]
fn the_default_palette_is_the_terminals_own_colours() {
    let terminal = &built_in()[0];
    assert_eq!(terminal.background, Color::Reset);
    assert_eq!(terminal.text, Color::Reset);
}

#[test]
fn an_unknown_or_empty_name_gives_the_default_palette() {
    let palettes = built_in();
    assert_eq!(named(&palettes, "no-such-palette"), 0);
    assert_eq!(named(&palettes, ""), 0);
}

#[test]
fn a_palette_is_found_whatever_the_case_and_with_underscores() {
    assert_eq!(palette("Nord").name, "nord");
    assert_eq!(palette(" GRUVBOX_LIGHT ").name, "gruvbox-light");
}

#[test]
fn palette_names_are_unique() {
    let palettes = built_in();
    let names: HashSet<&str> = palettes.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names.len(), palettes.len());
}

#[test]
fn every_palette_but_the_default_paints_its_own_background_and_text() {
    for palette in &built_in()[1..] {
        assert_ne!(palette.background, Color::Reset, "{}", palette.name);
        assert_ne!(palette.text, Color::Reset, "{}", palette.name);
        assert_ne!(palette.selection, palette.background, "{}", palette.name);
    }
}

#[test]
fn each_palette_keeps_its_primary_secondary_and_accent_colours_apart() {
    for p in built_in() {
        assert_ne!(p.primary, p.secondary, "{}", p.name);
        assert_ne!(
            p.secondary, p.accent,
            "{}: categories would look like links",
            p.name
        );
        assert_ne!(p.secondary, p.danger, "{}: `danger` must stand out", p.name);
    }
}

#[test]
fn there_are_fifteen_themes_with_at_most_two_variants_besides_their_accessible_ones() {
    // a theme's variants share the name's first word: tokyo-night, tokyo-night-day;
    // a version for colour blindness or low vision is not one more variant
    let accessible = [
        "-deuteranopia",
        "-tritanopia",
        "-colorblind",
        "-high-contrast",
    ];
    let mut count: HashMap<String, Vec<String>> = HashMap::new();
    for palette in &built_in()[1..] {
        let base = accessible
            .iter()
            .fold(palette.name.clone(), |name, end| name.replace(end, ""));
        let theme = base.split('-').next().unwrap_or_default().to_owned();
        let variants = count.entry(theme).or_default();
        if !variants.contains(&base) {
            variants.push(base);
        }
    }
    assert_eq!(count.len(), 15, "{count:?}");
    assert!(count.values().all(|v| v.len() <= 2), "{count:?}");
}

#[test]
fn colours_are_hex_or_the_terminals_names() {
    assert_eq!(colour("#2E3440"), Some(Color::Rgb(0x2e, 0x34, 0x40)));
    assert_eq!(colour("Cyan"), Some(Color::Cyan));
    assert_eq!(colour("reset"), Some(Color::Reset));
    assert_eq!(colour("#12345"), None);
    assert_eq!(colour("purple-ish"), None);
}

#[test]
fn a_palette_file_with_a_key_missing_says_which() {
    let text = NORD_LIKE.replace("warning = yellow\n", "");
    assert_eq!(parse("x", &text).unwrap_err(), "`warning` is missing");
    let text = NORD_LIKE.replace("code = green", "code = greenish");
    assert_eq!(
        parse("x", &text).unwrap_err(),
        "`code = greenish` is not a colour"
    );
}

#[test]
fn your_palettes_are_added_and_replace_a_built_in_one_of_the_same_name() {
    let dir = scratch("yours");
    fs::write(dir.join("mine.palette"), NORD_LIKE).unwrap();
    fs::write(dir.join("nord.palette"), NORD_LIKE).unwrap();
    fs::write(dir.join("broken.palette"), "text = #000000\n").unwrap(); // skipped
    fs::write(dir.join("notes.txt"), NORD_LIKE).unwrap(); // not a palette
    let mut palettes = built_in();
    let before = palettes.len();
    add_from(&mut palettes, &dir);
    assert_eq!(palettes.len(), before + 1);
    assert_eq!(palettes.last().unwrap().name, "mine");
    assert_eq!(
        palettes[named(&palettes, "nord")].background,
        Color::Rgb(0, 0, 0)
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn a_missing_folder_of_palettes_is_no_error() {
    let mut palettes = built_in();
    add_from(&mut palettes, std::path::Path::new("/no/such/folder"));
    assert_eq!(palettes.len(), built_in().len());
}

#[test]
fn the_default_palette_dims_and_reverses_as_lore_always_did() {
    let terminal = &built_in()[0];
    assert!(terminal.muted().add_modifier.contains(Modifier::DIM));
    assert!(
        terminal
            .selected()
            .add_modifier
            .contains(Modifier::REVERSED)
    );
    let nord = palette("nord");
    assert_eq!(nord.muted().fg, Some(nord.muted));
    assert_eq!(nord.selected().bg, Some(nord.selection));
}

// ── the picker (src/app/, src/interface/input.rs) ─────────────────────────

fn app() -> App {
    App::new(vec![Command::new("ls", Kind::Program)])
}

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn p_opens_the_picker_only_on_the_about_screen_and_types_in_the_search() {
    let p = key(KeyCode::Char(PALETTE_KEY));
    assert_eq!(action_for(p, Mode::About), Some(Action::OpenPalettes));
    assert_eq!(action_for(p, Mode::Browse), None);
    assert_eq!(action_for(p, Mode::Info), None);
    assert_eq!(action_for(p, Mode::Search), Some(Action::Type(PALETTE_KEY)));
}

#[test]
fn in_the_picker_arrows_try_enter_keeps_and_esc_cancels() {
    let picker = |code| action_for(key(code), Mode::Palette);
    assert_eq!(picker(KeyCode::Up), Some(Action::PreviousPalette));
    assert_eq!(picker(KeyCode::Down), Some(Action::NextPalette));
    assert_eq!(picker(KeyCode::Enter), Some(Action::KeepPalette));
    assert_eq!(picker(KeyCode::Esc), Some(Action::CancelPalette));
    assert_eq!(picker(KeyCode::Char('/')), None);
}

#[test]
fn trying_palettes_wraps_round_both_ways() {
    let mut app = app();
    let last = app.palettes.len() - 1;
    app.open_palettes();
    app.step_palette(-1);
    assert_eq!(app.chosen, last);
    app.step_palette(1);
    assert_eq!(app.chosen, 0);
    app.step_palette(2);
    assert_eq!(app.chosen, 2);
}

#[test]
fn esc_in_the_picker_puts_back_the_palette_from_before() {
    let mut app = app();
    app.open_palettes();
    app.step_palette(3);
    app.cancel_palettes();
    assert_eq!(app.palette().name, "terminal");
    assert_eq!(app.mode, Mode::Browse);
    assert!(app.picker.is_none());
}

#[test]
fn keeping_a_palette_returns_to_the_page_it_was_opened_from() {
    let mut app = app();
    app.open_info();
    app.open_palettes();
    assert!(app.page_shown(), "the page stays behind the picker");
    app.step_palette(1);
    app.close_palettes();
    assert_eq!(app.mode, Mode::Info);
    assert_eq!(app.chosen, 1);
}

#[test]
fn a_failed_save_keeps_the_picker_open_and_says_why() {
    let mut app = app();
    app.open_palettes();
    app.palette_failed("read-only file system");
    assert_eq!(app.mode, Mode::Palette);
    assert_eq!(
        app.picker.unwrap().note.as_deref(),
        Some("read-only file system")
    );
}
