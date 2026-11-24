//! Tests for the about screen and the logo (src/app.rs, src/interface/input.rs).

use std::collections::HashMap;

use lore_tui::app::{App, Mode, Setting};
use lore_tui::commands::{Command, Kind};
use lore_tui::compile_config::{ABOUT_KEY, ABOUT_SETTING_NAMES, LOGO, LORE_ORANGE, PALETTE_KEY};
use lore_tui::interface::input::{Action, action_for};
use lore_tui::utils::text;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::Color;

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn plus_opens_the_about_screen_from_the_list_and_a_page_but_types_in_the_search() {
    let plus = key(KeyCode::Char(ABOUT_KEY));
    assert_eq!(action_for(plus, Mode::Browse), Some(Action::OpenAbout));
    assert_eq!(action_for(plus, Mode::Info), Some(Action::OpenAbout));
    assert_eq!(
        action_for(plus, Mode::Search),
        Some(Action::Type(ABOUT_KEY))
    );
}

#[test]
fn plus_left_or_esc_leave_the_about_screen_and_other_keys_do_nothing() {
    for code in [KeyCode::Char(ABOUT_KEY), KeyCode::Left, KeyCode::Esc] {
        assert_eq!(action_for(key(code), Mode::About), Some(Action::CloseAbout));
    }
    assert_eq!(action_for(key(KeyCode::Char('x')), Mode::About), None);
}

#[test]
fn on_the_about_screen_the_arrows_choose_a_setting_and_enter_changes_it() {
    assert_eq!(
        action_for(key(KeyCode::Up), Mode::About),
        Some(Action::PreviousSetting)
    );
    assert_eq!(
        action_for(key(KeyCode::Down), Mode::About),
        Some(Action::NextSetting)
    );
    assert_eq!(
        action_for(key(KeyCode::Enter), Mode::About),
        Some(Action::ChangeSetting)
    );
    assert_eq!(
        action_for(key(KeyCode::Char(PALETTE_KEY)), Mode::About),
        Some(Action::OpenPalettes)
    );
}

#[test]
fn the_settings_are_in_this_order_and_choosing_wraps_round() {
    let mut app = App::new(vec![Command::new("ls", Kind::Program)]);
    assert_eq!(
        Setting::all(),
        [
            Setting::Colours,
            Setting::Internet,
            Setting::History,
            Setting::FastScroll,
            Setting::Reading,
            Setting::Favourite,
            Setting::Reset
        ]
    );
    assert_eq!(Setting::all().len(), ABOUT_SETTING_NAMES.len());
    assert_eq!(app.chosen_setting(), Setting::Colours);
    app.step_setting(1);
    assert_eq!(app.chosen_setting(), Setting::Internet);
    app.step_setting(-2);
    assert_eq!(app.chosen_setting(), Setting::Reset);
    app.step_setting(1);
    assert_eq!(app.chosen_setting(), Setting::Colours);
}

#[test]
fn the_palette_picker_opens_over_the_about_screen_and_goes_back_to_it() {
    let mut app = App::new(vec![Command::new("ls", Kind::Program)]);
    app.open_about();
    app.open_palettes();
    assert_eq!(app.mode, Mode::Palette);
    assert!(app.about_shown(), "the about screen stays under the picker");
    app.step_palette(1);
    app.cancel_palettes();
    assert_eq!(app.mode, Mode::About);
    assert_eq!(app.chosen, 0, "Esc puts the palette from before back");
    app.open_palettes();
    app.step_palette(1);
    app.close_palettes();
    assert_eq!(app.mode, Mode::About);
    assert_eq!(app.chosen, 1);
}

#[test]
fn turning_the_history_off_forgets_the_counts_and_the_unused_marks() {
    let mut app = App::new(vec![
        Command::new("ls", Kind::Program),
        Command::new("tar", Kind::Program),
    ]);
    app.set_history(true, HashMap::from([("ls".to_owned(), 3)]));
    assert!(app.history);
    assert!(app.is_unused("tar"));
    app.set_history(false, HashMap::from([("ls".to_owned(), 3)]));
    assert!(!app.history);
    assert!(app.uses.is_empty());
    assert!(!app.is_unused("tar"), "no history: nothing is marked");
}

#[test]
fn the_about_screen_goes_back_to_where_it_was_opened() {
    let mut app = App::new(vec![Command::new("ls", Kind::Program)]);
    app.open_info();
    app.open_about();
    assert_eq!(app.mode, Mode::About);
    assert!(!app.page_shown(), "the about screen takes the whole body");
    app.close_about();
    assert_eq!(app.mode, Mode::Info);
}

#[test]
fn the_logo_is_four_rows_that_fit_beside_the_search_bar() {
    assert_eq!(LOGO.len(), 4);
    assert!(LOGO.iter().all(|line| text::width(line) <= 32));
}

#[test]
fn the_logo_is_in_lores_orange() {
    assert_eq!(LORE_ORANGE, Color::Rgb(0xe8, 0x79, 0x3a));
}

#[test]
fn the_built_in_translations_are_in_41_languages() {
    assert_eq!(lore_tui::info::content::translated_languages(), 41);
}

#[test]
fn enter_on_fast_scroll_goes_round_its_choices_from_ten() {
    use lore_tui::compile_config::{FAST_SCROLL_CHOICES, FAST_SCROLL_DEFAULT};
    let mut app = App::new(vec![Command::new("ls", Kind::Program)]);
    assert_eq!(app.fast_scroll, FAST_SCROLL_DEFAULT);
    assert_eq!(FAST_SCROLL_DEFAULT, 10);
    let mut seen = Vec::new();
    for _ in 0..FAST_SCROLL_CHOICES.len() {
        app.next_fast_scroll();
        seen.push(app.fast_scroll);
    }
    assert_eq!(seen, [15, 20, 30, 5, 10]);
    app.fast_scroll = 7; // a hand-edited file: back to the first choice
    app.next_fast_scroll();
    assert_eq!(app.fast_scroll, 5);
}

#[test]
fn a_settings_picker_moves_jumps_by_letter_and_keeps_or_cancels() {
    let mut app = App::new(vec![Command::new("ls", Kind::Program)]);
    app.open_about();
    let items = ["none", "Go", "Haskell", "Python", "Perl"]
        .map(str::to_owned)
        .to_vec();
    app.open_choice(Setting::Favourite, items.clone(), "Haskell");
    assert_eq!(app.mode, Mode::Choose);
    assert!(app.about_shown(), "the about screen stays under the picker");
    app.jump_choice('p');
    assert_eq!(app.choice.as_ref().unwrap().at, 3);
    app.jump_choice('P');
    assert_eq!(
        app.choice.as_ref().unwrap().at,
        4,
        "the next one starting with it"
    );
    app.step_choice(10);
    assert_eq!(app.choice.as_ref().unwrap().at, 4, "stops at the end");
    assert_eq!(
        app.close_choice(true),
        Some((Setting::Favourite, "Perl".to_owned()))
    );
    assert_eq!(app.mode, Mode::About);
    app.open_choice(Setting::Favourite, items, "Go");
    assert_eq!(app.close_choice(false), None, "Esc keeps nothing");
}

#[test]
fn the_reading_choices_start_with_system_and_english_then_every_translation() {
    let choices = lore_tui::reading_choices();
    assert_eq!(&choices[..2], ["system", "en"]);
    assert!(choices.iter().any(|c| c == "pt_BR"));
    assert!(
        choices.iter().any(|c| c == "zh_CN"),
        "a code only the language pages have"
    );
    let mut sorted = choices[2..].to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted, choices[2..]);
}

#[test]
fn reset_all_asks_first_and_moving_away_keeps_everything() {
    let mut app = App::new(vec![Command::new("ls", Kind::Program)]);
    app.open_about();
    app.step_setting(-1);
    assert_eq!(app.chosen_setting(), Setting::Reset);
    app.reset_armed = true; // the first Enter (lib.rs)
    app.step_setting(-1);
    assert!(!app.reset_armed, "another setting chosen: nothing is reset");
    app.reset_armed = true;
    app.close_about();
    assert!(!app.reset_armed, "leaving the screen: nothing is reset");
}

#[test]
fn erasing_takes_files_and_folders_and_skips_what_is_not_there() {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("erase");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("docs/python")).unwrap();
    std::fs::write(dir.join("config"), "palette = nord\n").unwrap();
    std::fs::write(dir.join("docs/python/pages"), "x").unwrap();
    let failed =
        lore_tui::settings::erase(&[dir.join("config"), dir.join("docs"), dir.join("missing")]);
    assert!(failed.is_empty());
    assert!(!dir.join("config").exists() && !dir.join("docs").exists());
}

#[test]
fn the_about_screen_asks_to_follow_the_author() {
    use lore_tui::compile_config::ABOUT_FOLLOW;
    assert!(
        ABOUT_FOLLOW
            .iter()
            .all(|(_, link)| link.starts_with("https://"))
    );
    let sites: Vec<&str> = ABOUT_FOLLOW.iter().map(|(site, _)| *site).collect();
    assert_eq!(sites, ["GitHub", "LinkedIn", "X"]);
}
