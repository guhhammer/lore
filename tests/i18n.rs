//! The interface in the reader's language (src/utils/i18n.rs, i18n/). Its
//! own test file (its own process), since it sets the language and turns
//! the translation on for the whole program.

use std::collections::BTreeSet;
use std::sync::atomic::Ordering;

use lore_tui::app::App;
use lore_tui::commands::{Command, Kind};
use lore_tui::configs::{
    HEADING_STEPS, HEADING_VERSION, KEYS_BROWSE, READING_LANGUAGE, TRANSLATE_INTERFACE,
};
use lore_tui::info::page::{facts, render};
use lore_tui::info::recipes;
use lore_tui::interface::section::Section;
use lore_tui::utils::i18n::{self, catalog_of, codes, parse, recipe_name, shown, tr};

/// Every test here changes the same two switches, so one test runs them
/// in order instead of letting them race.
#[test]
fn the_screen_follows_the_language_and_what_lore_prints_stays_english() {
    *READING_LANGUAGE.write().unwrap() = Some("pt_BR".to_owned());

    // off (lore -p, search, explain): English, whatever the language
    TRANSLATE_INTERFACE.store(false, Ordering::Relaxed);
    assert_eq!(tr(HEADING_VERSION), "VERSION");
    assert_eq!(recipe_name("compress a folder"), "compress a folder");
    let tar = Command::new("tar", Kind::Program);
    let printed = render(&tar, &facts(&tar), None, 80);
    assert!(
        printed.iter().any(|l| l.text == "VERSION"),
        "-p stays English"
    );

    // on (the full-screen app): the chosen language
    TRANSLATE_INTERFACE.store(true, Ordering::Relaxed);
    assert_eq!(tr(HEADING_VERSION), "VERSÃO");
    let shown_page = render(&tar, &facts(&tar), None, 80);
    assert!(
        shown_page.iter().any(|l| l.text == "VERSÃO"),
        "the screen's page is"
    );
    let bar = i18n::keys(KEYS_BROWSE);
    assert!(bar.contains(&("/", "Buscar")), "{bar:?}");

    // a recipe: shown, and its page titled, by its translated name; its
    // name stays English (bookmarks, lore -p)
    let recipe = Command::new("compress a folder", Kind::Recipe);
    assert_eq!(shown(&recipe), "compactar uma pasta");
    assert_eq!(recipe.name, "compress a folder");
    let page = render(&recipe, &facts(&recipe), None, 80);
    assert_eq!(page[0].text, "compactar uma pasta");
    assert!(page.iter().any(|l| l.text == tr(HEADING_STEPS)));
    assert_eq!(shown(&Command::new("tar", Kind::Program)), "tar");

    // the search finds a recipe by its translated name, and still by its
    // English one
    let mut app = App::new(recipes::commands());
    for c in "compactar uma".chars() {
        app.type_char(c);
    }
    assert_eq!(
        app.selected().map(|c| c.name.as_str()),
        Some("compress a folder"),
        "found by its Portuguese name"
    );
    let mut app = App::new(recipes::commands());
    for c in "compress a fol".chars() {
        app.type_char(c);
    }
    assert_eq!(
        app.selected().map(|c| c.name.as_str()),
        Some("compress a folder"),
        "found by its English name"
    );

    // a language with no interface file, and English: English
    *READING_LANGUAGE.write().unwrap() = Some("sw".to_owned());
    assert_eq!(tr(HEADING_VERSION), "VERSION");
    *READING_LANGUAGE.write().unwrap() = Some("en".to_owned());
    assert_eq!(tr(HEADING_VERSION), "VERSION");

    TRANSLATE_INTERFACE.store(false, Ordering::Relaxed);
    *READING_LANGUAGE.write().unwrap() = None;
}

#[test]
fn every_language_translates_the_same_texts_and_every_recipe() {
    let reference: BTreeSet<String> = catalog_of("pt_BR").0.into_keys().collect();
    assert!(reference.len() > 200, "{}", reference.len());
    let names: BTreeSet<String> = recipes::commands().into_iter().map(|c| c.name).collect();
    for code in codes() {
        let (interface, recipes) = catalog_of(code);
        let keys: BTreeSet<String> = interface.into_keys().collect();
        assert_eq!(keys, reference, "{code}: not the same texts as pt_BR");
        let translated: BTreeSet<String> = recipes.into_keys().collect();
        assert_eq!(
            translated, names,
            "{code}: not every recipe, or one that is gone"
        );
    }
}

#[test]
fn every_translated_text_is_still_one_of_lores() {
    // the configs' source, Rust's `\` line continuations joined, so a text
    // split over lines is found whole
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/configs");
    let mut source = String::new();
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        source.push_str(&std::fs::read_to_string(entry.path()).unwrap());
    }
    let mut joined = String::new();
    let mut lines = source.lines().peekable();
    while let Some(line) = lines.next() {
        match line.strip_suffix('\\') {
            Some(start) => {
                joined.push_str(start);
                if let Some(next) = lines.peek_mut() {
                    *next = next.trim_start();
                }
            }
            None => {
                joined.push_str(line);
                joined.push('\n');
            }
        }
    }
    for english in catalog_of("pt_BR").0.keys() {
        let literal = english.replace('"', "\\\"").replace('\n', "\\n");
        assert!(
            joined.contains(&format!("\"{literal}\"")),
            "{english:?} is in i18n/ but no longer a text of lore's: update the files"
        );
    }
}

#[test]
fn a_translation_file_reads_escapes_and_skips_comments() {
    let catalog = parse("# a comment\n\na\\nb\tc\\nd\nx\ty\\\\z\n");
    assert_eq!(catalog.get("a\nb").map(String::as_str), Some("c\nd"));
    assert_eq!(catalog.get("x").map(String::as_str), Some("y\\z"));
    assert_eq!(catalog.len(), 2);
}

#[test]
fn a_translated_name_goes_under_its_own_capital_letter() {
    assert_eq!(Section::of("écrire"), Section::Letter('É'));
    assert_eq!(Section::of("сжать папку"), Section::Letter('С'));
    assert_eq!(Section::of("压缩文件夹"), Section::Symbol, "no capitals: #");
    assert_eq!(Section::of("tar"), Section::Letter('T'));
}
