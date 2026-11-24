//! Tests for copying an example (src/utils/clipboard.rs, the copy list in
//! src/app/ and src/interface/input.rs).

use lore_tui::app::{App, Mode, Page};
use lore_tui::commands::{Command, Kind};
use lore_tui::configs::{COPY_KEY, LANGUAGE_NO_COPY};
use lore_tui::info::content::parse;
use lore_tui::info::page::Facts;
use lore_tui::interface::input::{Action, action_for};
use lore_tui::utils::clipboard::{base64, osc52};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

/// An app reading tar's page, with `usage` as its examples.
fn reading(usage: &str) -> App {
    let tar = Command::new("tar", Kind::Program);
    let mut app = App::new(vec![tar.clone()]);
    app.open_info();
    let content = parse(&format!("# tar\n\nArchives.\n\n## Usage\n{usage}"));
    app.page = Some(Page {
        command: tar,
        width: 80,
        facts: Facts {
            content: Some(content),
            ..Facts::default()
        },
        lines: Vec::new(),
    });
    app
}

#[test]
fn base64_pads_as_the_standard_says() {
    assert_eq!(base64(b""), "");
    assert_eq!(base64(b"f"), "Zg==");
    assert_eq!(base64(b"fo"), "Zm8=");
    assert_eq!(base64(b"foo"), "Zm9v");
    assert_eq!(
        base64("tar -cf a.tar ~/é".as_bytes()),
        "dGFyIC1jZiBhLnRhciB+L8Op"
    );
}

#[test]
fn the_copy_is_an_osc_52_escape_the_terminal_carries_over_ssh() {
    assert_eq!(osc52("ls"), "\x1b]52;c;bHM=\x07");
}

#[test]
fn c_on_a_page_opens_the_examples_and_does_nothing_in_the_list() {
    let c = key(KeyCode::Char(COPY_KEY));
    assert_eq!(action_for(c, Mode::Info), Some(Action::OpenCopy));
    assert_eq!(action_for(c, Mode::Browse), None);
    assert_eq!(action_for(c, Mode::Search), Some(Action::Type(COPY_KEY)));
}

#[test]
fn in_the_copy_list_arrows_choose_enter_copies_and_esc_closes() {
    let list = |code| action_for(key(code), Mode::Copy);
    assert_eq!(list(KeyCode::Up), Some(Action::PreviousExample));
    assert_eq!(list(KeyCode::Down), Some(Action::NextExample));
    assert_eq!(list(KeyCode::Enter), Some(Action::CopyExample));
    assert_eq!(list(KeyCode::Esc), Some(Action::CloseCopy));
}

#[test]
fn choosing_an_example_wraps_round_and_copying_says_what_was_copied() {
    let mut app = reading("- `tar -cf a.tar dir` : pack\n- `tar -xf a.tar` : unpack\n");
    app.open_copy();
    assert_eq!(app.mode, Mode::Copy);
    app.step_example(-1);
    assert_eq!(app.chosen_example().as_deref(), Some("tar -xf a.tar"));
    app.step_example(1);
    let chosen = app.chosen_example();
    assert_eq!(chosen.as_deref(), Some("tar -cf a.tar dir"));
    app.close_copy(chosen.as_deref());
    assert_eq!(app.mode, Mode::Info);
    assert_eq!(app.note.as_deref(), Some("copied: tar -cf a.tar dir"));
}

#[test]
fn a_page_with_no_examples_says_so_instead_of_opening_the_list() {
    let mut app = reading("");
    app.open_copy();
    assert_eq!(app.mode, Mode::Info);
    assert!(app.note.is_some());
}

#[test]
fn on_a_language_page_c_copies_nothing_and_says_to_type_the_code() {
    let python = Command::new("Python", Kind::Language);
    let mut app = App::new(vec![python.clone()]);
    app.mode = Mode::Info;
    app.page = Some(Page {
        command: python,
        width: 80,
        facts: Facts::default(),
        lines: Vec::new(),
    });
    app.open_copy();
    assert_eq!(app.mode, Mode::Info, "no copy list opens");
    assert!(app.copies.is_empty());
    assert_eq!(app.note.as_deref(), Some(LANGUAGE_NO_COPY));
}

#[test]
fn a_piece_of_many_lines_is_named_by_its_first_and_how_many_follow() {
    assert_eq!(lore_tui::app::copy_label("ls -l"), "ls -l");
    assert_eq!(lore_tui::app::copy_label("a\nb"), "a  (+1 line)");
    assert_eq!(
        lore_tui::app::copy_label("x = 1\ny = 2\nz = 3"),
        "x = 1  (+2 lines)"
    );
}

#[test]
fn html_code_blocks_come_out_whole_and_as_typed() {
    let html = "<p>Use it:</p><pre>a = [1, 2]\n\tb = a &lt; 3</pre><p>or</p><pre>c()</pre>";
    let blocks = lore_tui::utils::html::code_blocks(html);
    assert_eq!(blocks, vec!["a = [1, 2]\n    b = a < 3", "c()"]);
}
