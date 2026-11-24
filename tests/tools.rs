//! Tests for the tools topic (src/info/tools.rs, tools/, their page).

use std::collections::HashSet;

use lore_tui::app::{App, Topic};
use lore_tui::commands::{Command, Kind};
use lore_tui::configs::{HEADING_MANUAL, HEADING_TASKS, HEADING_VERSION, TOOL_SUFFIX};
use lore_tui::info::check::{check, passes};
use lore_tui::info::content::parse;
use lore_tui::info::page::{Style, facts, render};
use lore_tui::info::tools::{commands, find};
use lore_tui::interface::rows::Row;

fn none_taken() -> HashSet<String> {
    HashSet::new()
}

#[test]
fn every_tool_page_passes_the_check_and_has_grouped_tasks() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tools");
    let mut count = 0;
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        let file = path.file_name().unwrap().to_string_lossy().into_owned();
        if !file.ends_with(".md") || file == "README.md" {
            continue;
        }
        count += 1;
        let text = std::fs::read_to_string(&path).unwrap();
        let name = text.lines().next().unwrap().trim_start_matches("# ");
        // the check takes the page's key: its file name, `docker-compose` for `# docker compose`
        let findings = check(file.trim_end_matches(".md"), text.as_bytes());
        assert!(passes(&findings), "{file}: {findings:?}");
        let content = parse(&text);
        assert!(!content.summary.is_empty(), "{file}: no summary");
        assert!(content.usage.len() >= 8, "{file}: fewer than 8 tasks");
        assert!(content.groups.len() >= 2, "{file}: fewer than 2 groups");
        assert_eq!(
            content.groups[0].0, 0,
            "{file}: a task before the first group"
        );
        assert!(!content.categories.is_empty(), "{file}: no categories");
        assert_eq!(name, name.to_lowercase(), "{file}: titles are lower case");
    }
    assert!(count >= 1);
    assert_eq!(
        count,
        commands(&none_taken()).len(),
        "every file is built in"
    );
}

#[test]
fn group_titles_inside_tasks_are_read_with_where_they_start() {
    let content =
        parse("# t\n\nA tool.\n\n## Tasks\n### One\n- `a` : x\n- `b` : y\n### Two\n- `c` : z\n");
    assert_eq!(content.usage.len(), 3);
    assert_eq!(
        content.groups,
        vec![(0, "One".to_owned()), (2, "Two".to_owned())]
    );
}

#[test]
fn a_tool_named_like_a_command_gets_a_suffix_and_still_opens() {
    let taken: HashSet<String> = ["curl".to_owned()].into();
    let tools = commands(&taken);
    let curl = tools.iter().find(|c| c.name.starts_with("curl")).unwrap();
    assert_eq!(curl.name, format!("curl{TOOL_SUFFIX}"));
    assert_eq!(curl.kind, Kind::Tool);
    assert!(find(&curl.name).is_some());
    assert!(find("curl").is_some());
}

#[test]
fn tools_are_a_topic_of_their_own_and_not_among_the_commands() {
    let mut list = vec![Command::new("ls", Kind::Program)];
    list.extend(commands(&none_taken()));
    let mut app = App::new(list);
    let names = |app: &App| -> Vec<String> {
        app.rows
            .iter()
            .filter_map(|row| match row {
                Row::Command(c) => Some(c.name.clone()),
                _ => None,
            })
            .collect()
    };
    assert_eq!(names(&app), ["ls"], "commands: no tools");
    app.show_topic(Topic::Tools);
    let shown = names(&app);
    assert_eq!(shown.len(), commands(&none_taken()).len());
    assert!(shown.contains(&"curl".to_owned()));
}

#[test]
fn a_tool_page_has_its_groups_as_headings_and_no_version_or_manual() {
    let tool = Command::new("curl", Kind::Tool);
    let lines = render(&tool, &facts(&tool), None, 80);
    let headings: Vec<&str> = lines
        .iter()
        .filter(|l| l.style == Style::Heading)
        .map(|l| l.text.as_str())
        .collect();
    assert!(headings.contains(&HEADING_TASKS));
    assert!(headings.contains(&"Download"), "{headings:?}");
    let texts: Vec<&str> = lines.iter().map(|l| l.text.as_str()).collect();
    assert!(!texts.contains(&HEADING_VERSION));
    assert!(!texts.contains(&HEADING_MANUAL));
    assert!(texts.iter().any(|t| t.contains("$ curl -O")));
}

#[test]
fn a_tool_page_is_found_by_what_it_says() {
    let mut list = vec![Command::new("ls", Kind::Program)];
    list.extend(commands(&none_taken()));
    let mut app = App::new(list);
    app.open_search();
    for c in "redirects".chars() {
        app.type_char(c);
    }
    assert!(
        app.rows
            .iter()
            .any(|r| matches!(r, Row::Command(c) if c.name == "curl"))
    );
}
