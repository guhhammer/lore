//! Tests for the links on every page (src/info/links.rs).

use lore_tui::info::links::{encode, searches};

#[test]
fn letters_digits_and_safe_marks_stay_and_the_rest_is_percent_encoded() {
    assert_eq!(encode("update-alternatives"), "update-alternatives");
    assert_eq!(encode("["), "%5B");
    assert_eq!(encode("a&b c"), "a%26b%20c");
}

#[test]
fn every_page_links_the_usual_help_sites_over_https_with_the_name_in_them() {
    let links = searches("ls");
    let labels: Vec<&str> = links.iter().map(|(label, _)| label.as_str()).collect();
    for site in [
        "Stack Overflow",
        "Unix & Linux Stack Exchange",
        "ArchWiki",
        "GeeksforGeeks",
        "Google",
    ] {
        assert!(labels.contains(&site), "{site} missing");
    }
    for (_, url) in &links {
        assert!(url.starts_with("https://") && url.contains("ls"), "{url}");
    }
}

#[test]
fn every_search_link_fits_a_panel_even_for_a_long_name() {
    for (_, url) in searches("update-alternatives") {
        assert!(url.len() <= 80, "{url} is {} long", url.len());
    }
}

#[test]
fn a_subcommand_is_searched_as_its_words() {
    for (_, url) in searches("git commit") {
        assert!(url.contains("git%20commit"), "{url}");
    }
}
