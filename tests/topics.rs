//! Tests for the topics: the bar under the search bar, ← → between them
//! (src/app/).

use lore_tui::app::{App, Topic};
use lore_tui::commands::{Command, Kind};
use lore_tui::configs::TOPIC_NAMES;
use lore_tui::info::recipes;

fn app_with_a_recipe() -> App {
    let mut list = vec![
        Command::new("ls", Kind::Program),
        Command::new("tar", Kind::Program),
    ];
    list.extend(recipes::commands().into_iter().take(1));
    App::new(list)
}

#[test]
fn every_topic_has_its_name_in_the_bar() {
    assert_eq!(Topic::all().len(), TOPIC_NAMES.len());
    assert_eq!(Topic::Commands.name(), "commands");
    assert_eq!(Topic::Bookmarks.name(), "bookmarks");
}

#[test]
fn the_list_opens_on_the_commands_topic() {
    let app = app_with_a_recipe();
    assert_eq!(app.topic, Topic::Commands);
    assert_eq!(
        app.topics,
        [Topic::Commands, Topic::Recipes, Topic::Docs, Topic::Tools],
        "docs and tools are always there"
    );
}

#[test]
fn right_and_left_go_round_the_topics_shown() {
    let mut app = app_with_a_recipe();
    app.step_topic(1);
    assert_eq!(app.topic, Topic::Recipes);
    app.step_topic(-2);
    assert_eq!(app.topic, Topic::Tools, "round to the last");
    app.step_topic(1);
    assert_eq!(app.topic, Topic::Commands, "and round to the first");
}

#[test]
fn a_topic_that_empties_gives_way_to_the_commands() {
    let mut app = app_with_a_recipe();
    app.toggle_bookmark(); // the selected command
    assert!(app.topics.contains(&Topic::Bookmarks));
    while app.topic != Topic::Bookmarks {
        app.step_topic(1);
    }
    app.toggle_bookmark(); // its only bookmark, taken away
    assert!(!app.topics.contains(&Topic::Bookmarks));
    assert_eq!(app.topic, Topic::Commands);
}

#[test]
fn lore_name_opens_on_the_topic_that_holds_it() {
    let mut app = app_with_a_recipe();
    let recipe = recipes::commands()[0].name.clone();
    assert!(app.open_direct(&recipe));
    assert_eq!(app.topic, Topic::Recipes);
    app.close_info();
    assert!(app.open_direct("tar"));
    assert_eq!(app.topic, Topic::Commands);
}
