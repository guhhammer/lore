//! Tests for what is marked dangerous (src/info/danger.rs).

use lore_tui::info::content::parse;
use lore_tui::info::danger::{command, example};

#[test]
fn examples_that_delete_or_overwrite_are_dangerous() {
    for line in [
        "rm -rf build",
        "sudo rm file",
        "dd if=disk.img of=/dev/sdb",
        "mkfs.ext4 /dev/sdb1",
        "git reset --hard HEAD~1",
        "git push --force",
        "find . -name '*.tmp' -delete",
        "echo x > /dev/sda",
    ] {
        assert!(example(line), "{line} should be dangerous");
    }
}

#[test]
fn ordinary_examples_are_not() {
    for line in [
        "ls -la",
        "git commit -m msg",
        "dd if=/dev/zero bs=1M count=1",
        "find . -name x",
        "address",
    ] {
        assert!(!example(line), "{line} should not be dangerous");
    }
}

#[test]
fn a_command_is_dangerous_when_its_page_says_so() {
    assert!(command(&parse(
        "# rm\n\nDeletes.\n\n## Categories\ndanger, files\n"
    )));
    assert!(!command(&parse("# ls\n\nLists.\n\n## Categories\nfiles\n")));
}

#[test]
fn rm_and_dd_pages_are_marked_dangerous() {
    use lore_tui::info::content::built_in;
    assert!(command(&parse(built_in("rm").unwrap())));
    assert!(command(&parse(built_in("dd").unwrap())));
}
