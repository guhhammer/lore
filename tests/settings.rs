//! Tests for the settings file (src/settings.rs).

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use lore_tui::settings::{get, path_from, set};

#[test]
fn the_file_is_in_xdg_config_home_when_it_is_set() {
    assert_eq!(
        path_from(Some(OsStr::new("/cfg")), Some(Path::new("/home/u"))),
        Some(PathBuf::from("/cfg/lore/config"))
    );
}

#[test]
fn without_xdg_config_home_the_file_is_in_dot_config() {
    assert_eq!(
        path_from(None, Some(Path::new("/home/u"))),
        Some(PathBuf::from("/home/u/.config/lore/config"))
    );
}

#[test]
fn a_relative_xdg_config_home_is_ignored_as_the_spec_says() {
    assert_eq!(
        path_from(Some(OsStr::new("cfg")), Some(Path::new("/home/u"))),
        Some(PathBuf::from("/home/u/.config/lore/config"))
    );
}

#[test]
fn with_no_home_and_no_xdg_there_is_no_file() {
    assert_eq!(path_from(None, None), None);
}

#[test]
fn a_setting_is_read_trimmed_and_comments_are_skipped() {
    let text = "# palette = dracula\n  palette =  nord  \nother = x\n";
    assert_eq!(get(text, "palette").as_deref(), Some("nord"));
    assert_eq!(get(text, "missing"), None);
}

#[test]
fn the_last_line_for_a_setting_wins() {
    assert_eq!(
        get("palette = nord\npalette = dracula\n", "palette").as_deref(),
        Some("dracula")
    );
}

#[test]
fn setting_a_value_replaces_its_line_and_keeps_every_other_line() {
    let text = "# mine\npalette = nord\nother = x\npalette = old\n";
    assert_eq!(
        set(text, "palette", "dracula"),
        "# mine\npalette = dracula\nother = x\n"
    );
}

#[test]
fn setting_a_new_value_adds_a_line_at_the_end() {
    assert_eq!(
        set("# mine\n", "palette", "nord"),
        "# mine\npalette = nord\n"
    );
    assert_eq!(set("", "palette", "nord"), "palette = nord\n");
}

// ── kept lists: bookmarks, recent pages ─────────────────────────────────────

use lore_tui::settings::{list_path_from, parse_list};

#[test]
fn lists_are_kept_in_xdg_data_home_or_dot_local_share() {
    assert_eq!(
        list_path_from(
            Some(OsStr::new("/data")),
            Some(Path::new("/home/u")),
            "bookmarks"
        ),
        Some(PathBuf::from("/data/lore/bookmarks"))
    );
    assert_eq!(
        list_path_from(None, Some(Path::new("/home/u")), "recent"),
        Some(PathBuf::from("/home/u/.local/share/lore/recent"))
    );
}

#[test]
fn a_list_is_one_name_a_line_and_blank_lines_are_skipped() {
    assert_eq!(parse_list("tar\n\n  git commit \n"), ["tar", "git commit"]);
}
