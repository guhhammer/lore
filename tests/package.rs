//! Tests for reading the package managers' answers (src/info/package.rs).

use std::path::Path;

use lore_tui::info::package::{apt_candidate, dpkg_owner, homebrew_cellar, pacman_owner};

#[test]
fn dpkg_gives_the_package_that_owns_the_file() {
    assert_eq!(
        dpkg_owner("coreutils: /usr/bin/ls\n").as_deref(),
        Some("coreutils")
    );
}

#[test]
fn dpkg_diversions_are_skipped_and_the_first_of_several_owners_is_kept() {
    let output = "diversion by dash from: /bin/sh\nbash, dash: /usr/bin/sh\n";
    assert_eq!(dpkg_owner(output).as_deref(), Some("bash"));
}

#[test]
fn apt_gives_its_candidate_unless_it_has_none() {
    let policy = "coreutils:\n  Installed: 9.7-3\n  Candidate: 9.7-4\n";
    assert_eq!(apt_candidate(policy).as_deref(), Some("9.7-4"));
    assert_eq!(apt_candidate("x:\n  Candidate: (none)\n"), None);
}

#[test]
fn pacman_gives_the_package_and_its_version() {
    assert_eq!(
        pacman_owner("/usr/bin/ls is owned by coreutils 9.5-1\n"),
        Some(("coreutils".to_owned(), "9.5-1".to_owned()))
    );
    assert_eq!(pacman_owner("error: No package owns /x\n"), None);
}

#[test]
fn a_homebrew_program_has_its_formula_and_version_in_its_path() {
    let path = Path::new("/opt/homebrew/Cellar/ripgrep/14.1.0/bin/rg");
    assert_eq!(
        homebrew_cellar(path),
        Some(("ripgrep".to_owned(), "14.1.0".to_owned()))
    );
    assert_eq!(homebrew_cellar(Path::new("/usr/bin/ls")), None);
}
