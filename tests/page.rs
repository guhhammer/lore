//! Tests for the page beside the list (src/info/page.rs): its sections,
//! its links and the note under them.

use std::path::PathBuf;

use lore_tui::commands::{Command, Kind};
use lore_tui::info::content::parse;
use lore_tui::info::package::Package;
use lore_tui::info::page::{Facts, LINK_WARNING, PageLine, Style, render};

fn texts(lines: &[PageLine]) -> Vec<&str> {
    lines.iter().map(|l| l.text.as_str()).collect()
}

fn ls_facts() -> Facts {
    Facts {
        content: Some(parse(
            "# ls\n\nLists files.\n\n## Usage\n- `ls -la` : every file\n\n## Links\n- GNU: https://www.gnu.org/software/coreutils/ls\n",
        )),
        whatis: Some("list directory contents".to_owned()),
        location: Some(PathBuf::from("/usr/bin/ls")),
        package: Some(Package {
            manager: "dpkg",
            name: "coreutils".to_owned(),
            installed: Some("9.7-3".to_owned()),
            newest: Some("9.7-4".to_owned()),
            update: "sudo apt install --only-upgrade coreutils".to_owned(),
        }),
        subcommands: Vec::new(),
    }
}

#[test]
fn the_page_has_its_sections_in_order() {
    let lines = render(&Command::new("ls", Kind::Program), &ls_facts(), None, 80);
    let headings: Vec<&str> = lines
        .iter()
        .filter(|l| l.style == Style::Heading)
        .map(|l| l.text.as_str())
        .collect();
    assert_eq!(headings, ["VERSION", "HOW TO USE", "LINKS", "MANUAL"]);
}

#[test]
fn the_version_section_says_when_an_update_is_available() {
    let lines = render(&Command::new("ls", Kind::Program), &ls_facts(), None, 80);
    let text = texts(&lines).join("\n");
    assert!(text.contains("coreutils (dpkg)"));
    assert!(text.contains("9.7-4: an update is available"));
    assert!(text.contains("/usr/bin/ls"));
}

#[test]
fn every_url_is_one_whole_line_even_on_a_narrow_panel() {
    let lines = render(&Command::new("ls", Kind::Program), &ls_facts(), None, 20);
    let urls: Vec<&str> = lines
        .iter()
        .filter(|l| l.style == Style::Url)
        .map(|l| l.text.trim())
        .collect();
    assert_eq!(urls[0], "https://www.gnu.org/software/coreutils/ls");
    assert_eq!(
        urls.len(),
        1 + lore_tui::compile_config::SEARCH_LINKS.len(),
        "its own link and every search"
    );
}

#[test]
fn the_safety_note_is_always_under_the_links() {
    for facts in [ls_facts(), Facts::default()] {
        let lines = render(&Command::new("zzz", Kind::Program), &facts, None, 200);
        let warning = lines
            .iter()
            .position(|l| l.style == Style::Warning)
            .unwrap();
        let last_url = lines.iter().rposition(|l| l.style == Style::Url).unwrap();
        assert!(warning > last_url);
        assert_eq!(lines[warning].text.trim(), LINK_WARNING);
    }
}

#[test]
fn a_command_without_a_page_falls_back_to_its_whatis_line() {
    let facts = Facts {
        whatis: Some("list directory contents".to_owned()),
        ..Facts::default()
    };
    let lines = render(&Command::new("ls", Kind::Program), &facts, None, 80);
    assert!(texts(&lines).contains(&"List directory contents."));
    assert!(
        texts(&lines)
            .iter()
            .any(|l| l.contains("No examples yet for ls"))
    );
}

#[test]
fn text_lines_fit_the_width_but_urls_may_not() {
    let lines = render(&Command::new("ls", Kind::Program), &ls_facts(), None, 30);
    for line in lines.iter().filter(|l| l.style != Style::Url) {
        assert!(line.text.chars().count() <= 30, "{:?}", line.text);
    }
}

#[test]
fn the_manual_comes_last() {
    let man = vec!["LS(1)".to_owned(), "NAME".to_owned()];
    let lines = render(
        &Command::new("ls", Kind::Program),
        &ls_facts(),
        Some(&man),
        80,
    );
    assert_eq!(texts(&lines[lines.len() - 2..]), ["LS(1)", "NAME"]);
}

#[test]
fn an_alias_says_what_it_runs_and_has_no_version_of_its_own() {
    let lines = render(
        &Command::alias("ll", "ls -alF"),
        &Facts::default(),
        None,
        80,
    );
    let text = texts(&lines).join("\n");
    assert!(text.contains("runs `ls -alF`"));
    assert!(text.contains("no version of its own"));
}

#[test]
fn a_program_with_subcommands_lists_them_under_their_own_heading() {
    let facts = Facts {
        subcommands: vec!["git commit".to_owned(), "git push".to_owned()],
        ..Facts::default()
    };
    let lines = render(&Command::new("git", Kind::Program), &facts, None, 80);
    let text = texts(&lines).join("\n");
    assert!(text.contains("SUBCOMMANDS"));
    assert!(text.contains("git commit, git push"));
}

#[test]
fn a_subcommand_says_whose_it_is() {
    let lines = render(
        &Command::new("git commit", Kind::Subcommand),
        &Facts::default(),
        None,
        80,
    );
    assert!(texts(&lines).contains(&"A subcommand of git."));
}

#[test]
fn the_categories_show_under_the_name() {
    let facts = Facts {
        content: Some(parse("# tar\n\nPacks.\n\n## Categories\narchives, files\n")),
        ..Facts::default()
    };
    let lines = render(&Command::new("tar", Kind::Program), &facts, None, 80);
    assert_eq!(lines[1].text, "[archives files]");
    assert_eq!(lines[1].style, Style::Categories);
}

#[test]
fn a_dangerous_command_warns_and_marks_its_dangerous_examples() {
    let facts = Facts {
        content: Some(parse(
            "# rm\n\nDeletes.\n\n## Usage\n- `rm -r dir` : delete a folder\n\n## Categories\ndanger, files\n",
        )),
        ..Facts::default()
    };
    let lines = render(&Command::new("rm", Kind::Program), &facts, None, 80);
    assert!(
        lines
            .iter()
            .any(|l| l.style == Style::Danger && l.text.contains("Can destroy data"))
    );
    assert!(
        lines
            .iter()
            .any(|l| l.style == Style::Danger && l.text.contains("! $ rm -r dir"))
    );
}

#[test]
fn related_commands_come_from_the_page_and_the_manuals_see_also() {
    use lore_tui::info::page::related;
    let facts = Facts {
        content: Some(parse("# tar\n\nPacks.\n\n## See also\n- zip\n- gzip\n")),
        ..Facts::default()
    };
    let man: Vec<String> = [
        "NAME",
        "       tar",
        "SEE ALSO",
        "       gzip(1), xz(1), tar(5)",
        "       License GPLv3+: GNU GPL version 3 or later",
        "AUTHOR",
        "       someone(1)",
    ]
    .iter()
    .map(|l| (*l).to_owned())
    .collect();
    assert_eq!(
        related(&Command::new("tar", Kind::Program), &facts, Some(&man)),
        ["zip", "gzip", "xz"]
    );
}
