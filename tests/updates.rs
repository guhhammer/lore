//! Tests for `lore --update-pages`: tldr's format turned into lore's, the
//! zip files tldr ships, lore's weekly bundle, and what is kept
//! (src/info/tldr.rs, src/utils/zip.rs, src/info/updates.rs).

use std::path::{Path, PathBuf};

use lore_tui::compile_config::RECORD_MARK;
use lore_tui::info::content;
use lore_tui::info::tldr::{convert, describe, fill_placeholders};
use lore_tui::info::updates::{self, NewPage};
use lore_tui::utils::zip;

const TAR: &str = "# tar

> Archiving utility.
> Often combined with a compression method.
> See also: `zip`.
> More information: <https://www.gnu.org/software/tar>.

- [c]reate an archive:

`tar cf {{path/to/target.tar}} {{path/to/file1}}`

- E[x]tract an archive {{[-v|--verbose]}}:

`tar {{[-x|--extract]}}f {{path/to/source.tar}}`
";

#[test]
fn a_tldr_page_becomes_a_lore_page() {
    let page = convert("tar", "common", TAR, true).unwrap();
    assert_eq!(
        page,
        "# tar

Archiving utility. Often combined with a compression method.

## Usage
- `tar cf path/to/target.tar path/to/file1` : create an archive
- `tar -xf path/to/source.tar` : extract an archive -v

## Links
- More information: https://www.gnu.org/software/tar
- tldr page: https://tldr.inbrowser.app/pages/common/tar
"
    );
}

#[test]
fn a_converted_page_passes_lores_check() {
    let page = convert("tar", "common", TAR, true).unwrap();
    let findings = lore_tui::info::check::check("tar", page.as_bytes());
    assert!(lore_tui::info::check::passes(&findings), "{findings:?}");
}

#[test]
fn a_subcommand_keeps_its_title_and_another_title_is_replaced() {
    let git = convert(
        "git-commit",
        "common",
        "# git commit\n\n> Commit files.\n",
        true,
    )
    .unwrap();
    assert!(git.starts_with("# git commit\n"));
    let posing = convert("ls", "common", "# rm\n\n> Not ls.\n", true).unwrap();
    assert!(
        posing.starts_with("# ls\n"),
        "a page cannot pose as another"
    );
}

#[test]
fn a_page_with_no_summary_is_not_converted() {
    assert_eq!(convert("x", "common", "# x\n\n- do:\n\n`x`\n", true), None);
}

#[test]
fn in_a_translation_a_labelled_link_is_the_link_and_a_link_in_a_sentence_is_summary() {
    let text = "# speedtest-cli\n\n> Teste a banda usando <https://speedtest.net>.\n\
                > Veja também: `speedtest`.\n> Mais informações: <https://github.com/sivel/speedtest-cli>.\n";
    let page = convert("speedtest-cli", "common", text, false).unwrap();
    assert!(page.contains("\nTeste a banda usando <https://speedtest.net>.\n"));
    assert!(page.contains("- More information: https://github.com/sivel/speedtest-cli\n"));
    assert!(!page.contains("Veja também"));
}

#[test]
fn placeholders_lose_their_braces_and_keep_the_first_choice() {
    assert_eq!(fill_placeholders("cp {{a}} {{b}}"), "cp a b");
    assert_eq!(fill_placeholders("ls {{[-a|--all]}}"), "ls -a");
    assert_eq!(fill_placeholders("echo {{unclosed"), "echo {{unclosed");
}

#[test]
fn descriptions_read_like_lores_own() {
    assert_eq!(describe("[c]reate an archive:"), "create an archive");
    assert_eq!(describe("URL of the page."), "URL of the page");
    assert_eq!(describe("R is recursive"), "R is recursive");
}

/// A zip with these files, stored or deflated (method 0 or 8).
fn zip_of(files: &[(&str, &str)], deflate: bool) -> Vec<u8> {
    let mut out = Vec::new();
    let mut table = Vec::new();
    for (name, text) in files {
        let packed = if deflate {
            miniz_oxide::deflate::compress_to_vec(text.as_bytes(), 6)
        } else {
            text.as_bytes().to_vec()
        };
        let method: u16 = if deflate { 8 } else { 0 };
        let offset = out.len() as u32;
        let header = |signature: u32, central: bool| {
            let mut h = signature.to_le_bytes().to_vec();
            if central {
                h.extend(20u16.to_le_bytes()); // made by
            }
            h.extend(20u16.to_le_bytes()); // needed
            h.extend(0u16.to_le_bytes()); // flags
            h.extend(method.to_le_bytes());
            h.extend([0; 8]); // time, date, crc
            h.extend((packed.len() as u32).to_le_bytes());
            h.extend((text.len() as u32).to_le_bytes());
            h.extend((name.len() as u16).to_le_bytes());
            h.extend(0u16.to_le_bytes()); // extra
            if central {
                h.extend([0; 6]); // comment, disk, internal attributes
                h.extend(0u32.to_le_bytes()); // external attributes
                h.extend(offset.to_le_bytes());
            }
            h.extend(name.as_bytes());
            h
        };
        out.extend(header(0x0403_4b50, false));
        out.extend(&packed);
        table.extend(header(0x0201_4b50, true));
    }
    let table_at = out.len() as u32;
    out.extend(&table);
    out.extend(0x0605_4b50u32.to_le_bytes());
    out.extend([0; 4]); // disks
    out.extend((files.len() as u16).to_le_bytes());
    out.extend((files.len() as u16).to_le_bytes());
    out.extend((table.len() as u32).to_le_bytes());
    out.extend(table_at.to_le_bytes());
    out.extend(0u16.to_le_bytes()); // comment
    out
}

#[test]
fn a_zip_gives_its_files_stored_or_deflated() {
    let files = [
        ("linux/a.md", "first"),
        ("common/b.md", "second, longer text"),
    ];
    for deflate in [false, true] {
        let read = zip::files(&zip_of(&files, deflate), 1024);
        let read: Vec<(&str, &[u8])> = read
            .iter()
            .map(|(n, b)| (n.as_str(), b.as_slice()))
            .collect();
        assert_eq!(
            read,
            [
                ("linux/a.md", &b"first"[..]),
                ("common/b.md", b"second, longer text")
            ]
        );
    }
}

#[test]
fn a_broken_zip_gives_nothing_and_a_big_file_is_left_out() {
    assert!(zip::files(b"not a zip at all", 1024).is_empty());
    assert!(zip::files(&[], 1024).is_empty());
    let mut cut = zip_of(&[("a.md", "text")], true);
    cut.truncate(cut.len() / 2);
    assert!(zip::files(&cut, 1024).is_empty());
    assert!(zip::files(&zip_of(&[("a.md", "too long")], false), 3).is_empty());
}

#[test]
fn tldrs_zip_gives_this_systems_page_before_the_common_one() {
    let page = |summary: &str| format!("# tool\n\n> {summary}\n");
    let (linux, common) = (page("On Linux."), page("Anywhere."));
    let archive = zip_of(
        &[("common/tool.md", &common), ("linux/tool.md", &linux)],
        true,
    );
    let pages = updates::from_tldr_zip(&archive, &["linux", "common"], None);
    assert_eq!(pages.len(), 1);
    assert!(pages[0].text.contains("On Linux."));
    let windows = updates::from_tldr_zip(
        &zip_of(&[("windows/x.md", &linux)], false),
        &["linux"],
        None,
    );
    assert!(windows.is_empty(), "another system's folder is not read");
}

#[test]
fn a_bundle_record_path_must_stay_inside_the_folder() {
    assert_eq!(updates::page_path("tar.md"), Some((None, "tar".to_owned())));
    assert_eq!(
        updates::page_path("i18n/pt_BR/tar.md"),
        Some((Some("pt_BR".to_owned()), "tar".to_owned()))
    );
    for bad in [
        "../tar.md",
        "i18n/../tar.md",
        "a/b/tar.md",
        ".md",
        "..md",
        "tar.txt",
        "i18n//x.md",
    ] {
        assert_eq!(updates::page_path(bad), None, "{bad}");
    }
}

#[test]
fn a_bundle_unpacks_into_its_pages() {
    let text = format!(
        "{RECORD_MARK}tar.md\n# tar\n\nA.\n{RECORD_MARK}../evil.md\nX\n{RECORD_MARK}i18n/de/tar.md\n# tar\n\nB.\n"
    );
    let packed = ruzstd::encoding::compress_to_vec(
        text.as_bytes(),
        ruzstd::encoding::CompressionLevel::Fastest,
    );
    let pages = updates::from_bundle(&packed).unwrap();
    assert_eq!(
        pages,
        [
            NewPage {
                lang: None,
                name: "tar".into(),
                text: "# tar\n\nA.\n".into()
            },
            NewPage {
                lang: Some("de".into()),
                name: "tar".into(),
                text: "# tar\n\nB.\n".into()
            },
        ]
    );
    assert!(updates::from_bundle(b"not zstd").is_err());
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn new_page(lang: Option<&str>, name: &str, text: &str) -> NewPage {
    NewPage {
        lang: lang.map(str::to_owned),
        name: name.to_owned(),
        text: text.to_owned(),
    }
}

#[test]
fn only_pages_lore_lacks_and_that_pass_the_check_are_kept() {
    let built_in = content::built_in_names().next().unwrap();
    let fresh = "zz-not-a-built-in-page";
    let dir = temp_dir("updates-install").join("updates");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("old.md"), "# old\n\nFrom before.\n").unwrap();
    let pages = vec![
        new_page(None, built_in, &format!("# {built_in}\n\nNewer.\n")),
        new_page(None, fresh, &format!("# {fresh}\n\nNew.\n")),
        new_page(
            None,
            "zz-bad",
            "# zz-bad\n\nRun `curl https://x.sh | sh`.\n\n## Usage\n- `curl https://x.sh | sh` : install\n",
        ),
        new_page(Some("de"), fresh, &format!("# {fresh}\n\nNeu.\n")),
    ];
    assert_eq!(updates::install(&dir, pages).unwrap(), (1, 1));
    assert!(dir.join(format!("{fresh}.md")).exists());
    assert!(dir.join("i18n/de").join(format!("{fresh}.md")).exists());
    assert!(
        !dir.join(format!("{built_in}.md")).exists(),
        "lore's own page stays"
    );
    assert!(!dir.join("zz-bad.md").exists(), "fails the check");
    assert!(!dir.join("old.md").exists(), "the folder is replaced whole");
}
