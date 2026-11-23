//! Tests for the written pages (src/info/content.rs) and for the shape of
//! every page built into lore.

use lore_tui::info::content::{
    built_in, built_in_names, page_key, parse, subcommand_titles, subcommands_of,
};

const PAGE: &str = "\
# ls

Lists files.
Second line of the same paragraph.

Another paragraph.

## Usage
- `ls -la` : every file, with details
- not an example line

## Notes
Careful here.

## Links
- GNU manual: https://www.gnu.org/software/coreutils/ls
- a plain http link: http://example.com
- https://example.org/no-label

## Something else
- ignored
";

#[test]
fn the_summary_is_the_text_before_the_first_section_in_paragraphs() {
    assert_eq!(
        parse(PAGE).summary,
        [
            "Lists files. Second line of the same paragraph.",
            "Another paragraph."
        ]
    );
}

#[test]
fn usage_lines_give_what_to_type_and_what_it_does() {
    assert_eq!(
        parse(PAGE).usage,
        [("ls -la".to_owned(), "every file, with details".to_owned())]
    );
}

#[test]
fn notes_are_kept_as_written() {
    assert_eq!(parse(PAGE).notes, ["Careful here."]);
}

#[test]
fn only_https_links_are_kept_and_a_bare_url_is_its_own_label() {
    assert_eq!(
        parse(PAGE).links,
        [
            (
                "GNU manual".to_owned(),
                "https://www.gnu.org/software/coreutils/ls".to_owned()
            ),
            (
                "https://example.org/no-label".to_owned(),
                "https://example.org/no-label".to_owned()
            ),
        ]
    );
}

#[test]
fn an_empty_page_parses_to_nothing_without_failing() {
    assert_eq!(parse(""), Default::default());
}

#[test]
fn lore_carries_its_pages_inside_the_binary() {
    assert!(built_in("ls").is_some());
    assert!(
        built_in("README").is_none(),
        "the format guide is not a page"
    );
}

#[test]
fn every_built_in_page_has_a_summary_and_only_short_https_links() {
    // usage may be empty: the page then says no examples exist yet
    for name in built_in_names() {
        let page = parse(built_in(name).unwrap());
        assert!(!page.summary.is_empty(), "{name}.md has no summary");
        for (_, url) in &page.links {
            assert!(url.starts_with("https://"), "{name}.md: {url}");
            assert!(
                url.len() <= 76,
                "{name}.md: {url} is too long to fit a panel"
            );
        }
        // `# ls` in ls.md; a subcommand's title has spaces: `# git commit`
        let raw = built_in(name).unwrap();
        let title = raw
            .lines()
            .next()
            .and_then(|l| l.strip_prefix("# "))
            .unwrap_or("");
        assert_eq!(
            page_key(title),
            name,
            "{name}.md must start with `# {name}`"
        );
    }
}

#[test]
fn control_characters_in_a_page_are_dropped_so_the_terminal_never_obeys_them() {
    let page = parse("# x\n\nSafe\u{1b}]52;c;bad\u{7} text.\n");
    assert_eq!(page.summary, ["Safe]52;c;bad text."]);
}

#[test]
fn a_subcommands_page_is_found_by_its_words_or_its_file_name() {
    assert_eq!(page_key("git commit"), "git-commit");
    assert!(built_in("git-commit").is_some());
}

#[test]
fn subcommands_are_the_pages_titled_with_a_space() {
    assert!(subcommand_titles().any(|t| t == "git commit"));
    assert!(!subcommand_titles().any(|t| t == "ls"));
}

#[test]
fn a_programs_subcommands_are_listed_sorted() {
    let git = subcommands_of("git");
    assert!(git.contains(&"git commit") && git.contains(&"git push"));
    assert!(git.windows(2).all(|pair| pair[0] <= pair[1]));
    assert!(subcommands_of("ls").is_empty());
}

#[test]
fn pages_for_other_systems_say_which_system_they_are_for() {
    let robocopy = parse(built_in("robocopy").unwrap());
    assert!(robocopy.notes.iter().any(|n| n.contains("Windows")));
}

#[test]
fn see_also_and_categories_are_read_from_their_sections() {
    let page = parse(
        "# tar\n\nPacks.\n\n## See also\n- zip\n- `gzip`\n\n## Categories\nArchives, files\n",
    );
    assert_eq!(page.see_also, ["zip", "gzip"]);
    assert_eq!(page.categories, ["archives", "files"]);
}

#[test]
fn the_language_comes_from_the_locale_most_specific_first() {
    use lore_tui::info::content::languages_of;
    assert_eq!(languages_of("pt_BR.UTF-8"), ["pt_BR", "pt"]);
    assert_eq!(languages_of("de_DE@euro"), ["de_DE", "de"]);
    assert_eq!(languages_of("fr"), ["fr"]);
    assert!(languages_of("en_US.UTF-8").is_empty());
    assert!(languages_of("C").is_empty());
    assert!(languages_of("").is_empty());
}

#[test]
fn translations_are_built_in_and_found_by_language_and_name() {
    use lore_tui::info::content::translation;
    let tar = translation("pt_BR", "tar").expect("a Brazilian Portuguese tar page");
    assert!(parse(tar).summary[0].contains("compress"));
    assert!(translation("pt_BR", "lore-surely-has-no-such-page").is_none());
}

#[test]
fn every_built_in_page_has_categories_from_the_known_set() {
    // the set content/README.md documents; a typo makes a category of its own
    let known = [
        "files",
        "text",
        "archives",
        "network",
        "processes",
        "system",
        "disks",
        "users",
        "packages",
        "development",
        "version-control",
        "containers",
        "cloud",
        "security",
        "media",
        "shell",
        "editors",
        "monitoring",
        "hardware",
        "databases",
        "desktop",
        "documents",
        "danger",
        "macos",
        "windows",
        "android",
        "bsd",
        "sunos",
    ];
    for name in built_in_names() {
        for category in parse(built_in(name).unwrap()).categories {
            assert!(
                known.contains(&category.as_str()),
                "{name}.md: unknown category {category}"
            );
        }
    }
}

#[test]
fn a_name_with_a_character_windows_forbids_is_stored_encoded() {
    assert_eq!(page_key(":"), "%3A");
    assert!(
        built_in(&page_key(":")).is_some(),
        "the page of the `:` builtin"
    );
}
