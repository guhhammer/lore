//! Pages in: the language pages are read in, set on the about screen. Its
//! own test file (its own process), since it sets the language for the
//! whole program.

use lore_tui::app::App;
use lore_tui::commands::{Command, Kind};
use lore_tui::compile_config::READING_LANGUAGE;
use lore_tui::info::content;

fn read_in(code: Option<&str>) {
    *READING_LANGUAGE.write().unwrap() = code.map(str::to_owned);
}

#[test]
fn the_search_matches_the_pages_in_the_language_read_else_in_english() {
    read_in(Some("pt_BR"));
    assert_eq!(
        content::languages(),
        ["pt_BR", "pt"],
        "Pages in wins over LANG"
    );
    let mut app = App::new(vec![Command::new("tar", Kind::Program)]);
    // a word of tar's Portuguese summary, not of its English one
    let summary = content::find("tar")
        .unwrap()
        .summary
        .join(" ")
        .to_lowercase();
    let word = summary
        .split(|c: char| !c.is_alphabetic())
        .find(|w| {
            w.len() >= 6
                && !content::built_in("tar")
                    .unwrap()
                    .to_lowercase()
                    .contains(*w)
        })
        .expect("a Portuguese word")
        .to_owned();
    for c in word.chars() {
        app.type_char(c);
    }
    assert_eq!(
        app.selected().map(|c| c.name.as_str()),
        Some("tar"),
        "found by {word}"
    );

    // English: the same word finds nothing, and English words find tar
    read_in(Some("en"));
    assert!(content::languages().is_empty());
    app.reload_docs();
    assert!(
        app.selected().is_none(),
        "{word} is not in the English page"
    );
    read_in(None);
}
