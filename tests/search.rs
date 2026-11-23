//! Tests for the search rules (src/interface/search.rs).
//! Each test name says the rule it checks.

use lore_tui::interface::search::{name_rank, split};

#[test]
fn an_exact_name_ranks_before_any_other_match() {
    assert!(name_rank("rm", "rm") < name_rank("rmdir", "rm"));
    assert!(
        name_rank("RM", "rm") == name_rank("rm", "rm"),
        "case is ignored"
    );
}

#[test]
fn a_name_starting_with_the_text_ranks_before_one_with_it_inside() {
    assert!(name_rank("rmdir", "rm") < name_rank("chrm", "rm"));
}

#[test]
fn the_text_at_a_word_start_inside_the_name_ranks_before_anywhere_else() {
    assert!(name_rank("git commit", "commit") < name_rank("xcommitx", "commit"));
    assert!(name_rank("apt-get", "get") < name_rank("forget", "get"));
}

#[test]
fn among_equal_matches_the_shorter_name_ranks_first() {
    assert!(name_rank("rmdir", "rm") < name_rank("rmmod-all", "rm"));
}

#[test]
fn a_name_without_the_text_does_not_match() {
    assert_eq!(name_rank("ls", "rm"), None);
}

#[test]
fn an_at_sign_first_asks_for_only_commands_with_a_lore_page() {
    assert_eq!(split("@gr"), (true, "gr"));
    assert_eq!(split("@"), (true, ""));
    assert_eq!(split("gr@"), (false, "gr@"));
}

use lore_tui::info::content::parse;
use lore_tui::interface::search::{Doc, Terms, content_score};

/// A page's score for a query; 0 when the query is too short to search by.
fn score(doc: &Doc, query: &str) -> u32 {
    Terms::new(query).map_or(0, |terms| content_score(doc, &terms))
}

/// A page's snippet for a query; None when the query is too short.
fn snip(doc: &Doc, query: &str) -> Option<String> {
    Terms::new(query).and_then(|terms| lore_tui::interface::search::snippet(doc, &terms))
}

fn doc(page: &str) -> Doc {
    Doc::of(&parse(page))
}

const TAR: &str = "# tar\n\nPacks files into an archive, and compresses it.\n\n## Usage\n- `tar czf a.tgz dir` : compress a folder\n\n## Categories\narchives, files\n";

#[test]
fn a_page_is_found_by_the_words_it_contains() {
    assert!(score(&doc(TAR), "compress folder") > 0);
    assert_eq!(
        score(&doc(TAR), "compress spreadsheet"),
        0,
        "every word must be there"
    );
}

#[test]
fn a_category_counts_more_than_a_word_in_the_text() {
    let by_category = score(&doc(TAR), "archives");
    let by_example = score(&doc(TAR), "czf");
    assert!(by_category > by_example);
}

#[test]
fn the_whole_phrase_as_typed_scores_more_than_its_words_apart() {
    let apart = doc("# x\n\nA folder to compress.\n");
    assert!(score(&doc(TAR), "compress a folder") > score(&apart, "compress a folder"));
}

#[test]
fn queries_too_short_do_not_search_pages() {
    assert_eq!(score(&doc(TAR), "ar"), 0);
}

#[test]
fn a_word_meaning_the_same_finds_the_page_but_counts_less_than_the_word_itself() {
    use lore_tui::interface::search::alternatives;
    assert!(alternatives("folder").iter().any(|w| w == "directory"));
    let page = doc("# x\n\nCompresses a directory.\n");
    let same = score(&page, "compress directory");
    let synonym = score(&page, "compress folder");
    assert!(synonym > 0 && synonym < same);
}

#[test]
fn a_plural_also_matches_its_singular() {
    use lore_tui::interface::search::alternatives;
    assert!(alternatives("files").contains(&"file".to_owned()));
    assert!(alternatives("directories").contains(&"directory".to_owned()));
    assert!(
        !alternatives("class").contains(&"clas".to_owned()),
        "a double s is not a plural"
    );
    assert!(score(&doc(TAR), "archive files") > 0);
}

#[test]
fn the_snippet_is_the_page_line_with_the_most_query_words_cut_around_them() {
    let cut = doc(
        "# cut\n\nCut out fields from STDIN or files, and print them in a long sentence that goes on.\n\n## Usage\n- `cut -c 1-10 file` : print a range of characters of each line of a file\n",
    );
    let found = snip(&cut, "cut files").unwrap();
    assert!(found.starts_with('[') && found.ends_with(']'), "{found}");
    assert!(
        found.contains("Cut out fields from STDIN or files"),
        "{found}"
    );
    assert!(found.ends_with(" ...]"), "cut at the end: {found}");
}

#[test]
fn no_snippet_when_the_page_has_none_of_the_words_or_the_query_is_short() {
    assert_eq!(snip(&doc(TAR), "spreadsheet"), None);
    assert_eq!(snip(&doc(TAR), "ar"), None);
}

#[test]
fn a_word_matches_at_the_start_of_words_only() {
    use lore_tui::interface::search::find_word;
    assert_eq!(find_word("cut out fields", "cut"), Some(0));
    assert_eq!(find_word("it cuts lines", "cut"), Some(3));
    assert_eq!(find_word("the cut-off", "off"), Some(8));
    assert_eq!(find_word("execute a file", "cut"), None);
}

#[test]
fn a_command_named_by_one_of_the_words_is_recognised() {
    assert!(Terms::new("cut files").unwrap().names("cut"));
    assert!(Terms::new("files CUT").unwrap().names("Cut"));
    assert!(!Terms::new("cutting files").unwrap().names("cut"));
}
