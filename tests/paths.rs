//! Tests for turning configured path patterns into real paths (src/paths.rs).
//! Each test name says the rule it checks.

use std::path::{Path, PathBuf};

use lore_tui::paths::expand;

/// A fresh folder holding these empty files, under cargo's temp dir.
fn temp_with(name: &str, files: &[&str]) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    for file in files {
        let path = dir.join(file);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, "").unwrap();
    }
    dir
}

#[test]
fn a_tilde_path_starts_at_the_home_directory() {
    let home = Path::new("/home/someone");
    assert_eq!(expand("~/.bashrc", Some(home)), [home.join(".bashrc")]);
}

#[test]
fn a_tilde_path_gives_nothing_when_there_is_no_home() {
    assert!(expand("~/.bashrc", None).is_empty());
}

#[test]
fn a_plain_path_is_kept_even_if_it_does_not_exist_for_the_caller_to_skip() {
    assert_eq!(
        expand("/nonexistent/x", None),
        [PathBuf::from("/nonexistent/x")]
    );
}

#[test]
fn a_star_matches_the_names_with_that_prefix_and_suffix_sorted() {
    let dir = temp_with("star", &["b.sh", "a.sh", "notes.txt"]);
    let pattern = format!("{}/*.sh", dir.display());
    assert_eq!(expand(&pattern, None), [dir.join("a.sh"), dir.join("b.sh")]);
}

#[test]
fn a_star_can_stand_for_a_directory_in_the_middle_of_a_path() {
    let dir = temp_with("mid", &["one/bin/x", "two/bin/y", "three/lib/z"]);
    let pattern = format!("{}/*/bin", dir.display());
    assert_eq!(
        expand(&pattern, None),
        [dir.join("one/bin"), dir.join("two/bin")]
    );
}

#[test]
fn a_star_in_a_directory_that_does_not_exist_gives_nothing() {
    assert!(expand("/nonexistent/*.sh", None).is_empty());
}
