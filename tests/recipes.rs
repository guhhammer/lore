//! Tests for the recipes (src/info/recipes.rs, recipes/, their page).

use lore_tui::app::{App, Topic};
use lore_tui::commands::{Command, Kind};
use lore_tui::configs::{HEADING_MANUAL, HEADING_STEPS, HEADING_VERSION};
use lore_tui::info::page::{facts, render};
use lore_tui::info::recipes::{commands, find, uses};
use lore_tui::interface::rows::Row;

#[test]
fn every_recipe_has_a_summary_steps_and_categories() {
    let all = commands();
    assert!(all.len() >= 12, "{}", all.len());
    for recipe in &all {
        let content = find(&recipe.name).unwrap();
        assert!(!content.summary.is_empty(), "{}: no summary", recipe.name);
        assert!(
            content.usage.len() >= 3,
            "{}: fewer than 3 steps",
            recipe.name
        );
        assert!(
            !content.categories.is_empty(),
            "{}: no categories",
            recipe.name
        );
        assert_eq!(
            recipe.name,
            recipe.name.to_lowercase(),
            "titles are lower case"
        );
    }
}

#[test]
fn the_asked_for_recipes_are_there() {
    for name in [
        "free disk space",
        "what is using a port",
        "why won't a service start",
        "add a user",
    ] {
        assert!(find(name).is_some(), "{name}");
    }
}

#[test]
fn recipes_are_a_topic_of_their_own_a_to_z_and_not_among_the_commands() {
    let mut list = vec![Command::new("ls", Kind::Program)];
    list.extend(commands().into_iter().take(2));
    let mut app = App::new(list);
    let labels = |app: &App| -> Vec<String> {
        app.rows
            .iter()
            .map(|row| match row {
                Row::Header(section) => format!("[{section}]"),
                Row::Title(title) => format!("[{title}]"),
                Row::Command(c) => c.name.clone(),
            })
            .collect()
    };
    assert_eq!(labels(&app), ["[L]", "ls"], "commands: no recipes");
    app.step_topic(1);
    assert_eq!(app.topic, Topic::Recipes);
    let shown = labels(&app);
    assert!(shown[0].starts_with('['), "under a letter: {shown:?}");
    assert_eq!(shown.iter().filter(|l| !l.starts_with('[')).count(), 2);
}

#[test]
fn a_recipe_is_found_by_what_its_page_says() {
    let mut list = vec![Command::new("ls", Kind::Program)];
    list.extend(commands());
    let mut app = App::new(list);
    app.open_search();
    for c in "journal".chars() {
        app.type_char(c);
    }
    assert!(
        app.rows
            .iter()
            .any(|r| matches!(r, Row::Command(c) if c.name == "free disk space"))
    );
}

#[test]
fn a_recipes_page_has_numbered_steps_and_no_version_or_manual() {
    let recipe = Command::new("free disk space", Kind::Recipe);
    let lines = render(&recipe, &facts(&recipe), None, 80);
    let texts: Vec<&str> = lines.iter().map(|l| l.text.as_str()).collect();
    assert!(texts.contains(&HEADING_STEPS));
    assert!(!texts.contains(&HEADING_VERSION));
    assert!(!texts.contains(&HEADING_MANUAL));
    assert!(texts.iter().any(|t| t.trim_start().starts_with("1. ")));
}

#[test]
fn a_recipe_uses_each_steps_commands_past_sudo_and_pipes() {
    let content = lore_tui::info::content::parse(
        "# x\n\nY.\n\n## Steps\n- `sudo ss -tulpn | grep :80` : a\n- `ss -s` : b\n",
    );
    assert_eq!(uses(&content), ["ss", "grep"]);
}

#[test]
fn a_recipe_does_not_count_the_shells_own_words_as_commands() {
    let content = lore_tui::info::content::parse(
        "# x\n\nY.\n\n## Steps\n- `while IFS= read -r line; do echo \"$line\"; done < f` : a\n- `[ -f x ] && cat x 2>&1` : b\n",
    );
    assert_eq!(uses(&content), ["read", "echo", "cat"]);
}

#[test]
fn there_are_a_hundred_recipes_and_every_step_line_is_read() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/recipes");
    let mut count = 0;
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|x| x != "md") || path.ends_with("README.md") {
            continue;
        }
        count += 1;
        let text = std::fs::read_to_string(&path).unwrap();
        let written = text
            .lines()
            .skip_while(|l| *l != "## Steps")
            .skip(1)
            .take_while(|l| !l.starts_with("## "))
            .filter(|l| l.starts_with("- `"))
            .count();
        let read = lore_tui::info::content::parse(&text).usage.len();
        assert_eq!(
            read,
            written,
            "{}: a step line was not read",
            path.display()
        );
    }
    assert_eq!(count, 100);
}
