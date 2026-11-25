//! Puts the default pages of `content/` inside the binary, so lore has them
//! wherever it runs, offline, however it was installed (a package, `cargo
//! install`, a copied file). Every page goes into one zstd-compressed text,
//! `$OUT_DIR/pages.zst` (19 MB of Markdown packs to about 3 MB), and
//! `$OUT_DIR/content.rs` holds the tables: EMBEDDED, (command name, where its
//! page sits in that text), and EMBEDDED_I18N, (language, name, where) from
//! `content/i18n/<language>/`, both sorted for binary search.
//!
//! And the recipes of `recipes/` (`$OUT_DIR/recipes.rs`: BUILT_IN_RECIPES,
//! each file's text), each checked for a title and steps.
//!
//! And learnxinyminutes.com's pages of `languages/` (`$OUT_DIR/languages.rs`:
//! LANGUAGE_PAGES, (file name, where), LANGUAGE_I18N, (language, file name,
//! where), and LANGUAGE_TEXT, packed like the pages: 13 MB to about 3 MB).
//!
//! It also puts in the colour palettes of `palettes/` (`$OUT_DIR/palettes.rs`:
//! BUILT_IN_PALETTES, (name, the file's text), `terminal` first), and checks
//! each one, so a palette with a missing key or a bad colour fails the build
//! with the file and the key named, not lore at run time.

use std::env;
use std::fs;
use std::path::Path;

/// zstd's strongest level before its "ultra" ones, which need more memory to
/// unpack: slow to build (only when a page changes), as fast to unpack.
const ZSTD_LEVEL: i32 = 19;

/// Appends the page at `path` to `text`; its (start, end) byte range there.
fn add(text: &mut String, path: &str) -> (usize, usize) {
    let start = text.len();
    text.push_str(&fs::read_to_string(path).expect("a page is UTF-8 text"));
    (start, text.len())
}

/// (name, path) of every page in `dir`, but README.md.
fn pages(dir: &Path) -> Vec<(String, String)> {
    let mut found = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(file) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        // README.md explains the format; it is not a command's page
        let Some(name) = file.strip_suffix(".md").filter(|c| *c != "README") else {
            continue;
        };
        found.push((name.to_owned(), path.display().to_string()));
    }
    found.sort(); // binary search needs it; also the same binary every time
    found
}

/// The keys every palette file has; `interface::palette` reads them (and
/// src/configs/'s `PALETTE_KEYS` lists them again, for the run time).
const PALETTE_KEYS: [&str; 10] = [
    "background",
    "text",
    "muted",
    "selection",
    "primary",
    "secondary",
    "accent",
    "danger",
    "code",
    "warning",
];

/// The colour names a palette can use besides `#rrggbb` (src/configs/'s
/// `COLOUR_NAMES`, by name).
const COLOUR_NAMES: [&str; 17] = [
    "reset",
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "cyan",
    "gray",
    "darkgray",
    "lightred",
    "lightgreen",
    "lightyellow",
    "lightblue",
    "lightmagenta",
    "lightcyan",
    "white",
];

/// Checks one palette file: every key there, every colour one lore knows.
fn check_palette(file: &str, text: &str) {
    for key in PALETTE_KEYS {
        let value = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.starts_with('#'))
            .filter_map(|line| line.split_once('='))
            .rfind(|(k, _)| k.trim() == key)
            .map(|(_, v)| v.trim())
            .unwrap_or_else(|| panic!("{file}: `{key}` is missing"));
        let hex = value.len() == 7
            && value.starts_with('#')
            && value[1..].chars().all(|c| c.is_ascii_hexdigit());
        assert!(
            hex || COLOUR_NAMES.contains(&value),
            "{file}: `{key} = {value}` is not #rrggbb or one of {COLOUR_NAMES:?}"
        );
    }
}

/// `$OUT_DIR/palettes.rs`: every `palettes/*.palette`, checked, `terminal`
/// first (the default), then by name.
fn palettes(out: &Path) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("palettes");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut found: Vec<(String, String)> = fs::read_dir(&dir)
        .expect("palettes/ is there")
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let name = path
                .file_name()?
                .to_str()?
                .strip_suffix(".palette")?
                .to_owned();
            Some((name, path.display().to_string()))
        })
        .collect();
    found.sort_by_key(|(name, _)| (name != "terminal", name.clone()));
    assert!(
        found.first().is_some_and(|(name, _)| name == "terminal"),
        "palettes/terminal.palette, the default, is missing"
    );
    let mut table = String::from("pub static BUILT_IN_PALETTES: &[(&str, &str)] = &[\n");
    for (name, path) in &found {
        check_palette(
            path,
            &fs::read_to_string(path).expect("a palette is UTF-8 text"),
        );
        table.push_str(&format!("    ({name:?}, include_str!({path:?})),\n"));
    }
    table.push_str("];\n");
    fs::write(out.join("palettes.rs"), table).expect("OUT_DIR is writable");
}

/// `$OUT_DIR/recipes.rs` and `tools.rs`: every `recipes/*.md` and
/// `tools/*.md` but README.md, checked: a `# title` first and at least one
/// step, or the build stops.
fn recipes(out: &Path) {
    task_pages(out, "recipes", "BUILT_IN_RECIPES");
    task_pages(out, "tools", "BUILT_IN_TOOLS");
}

fn task_pages(out: &Path, folder: &str, table_name: &str) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(folder);
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut table = format!("pub static {table_name}: &[&str] = &[\n");
    for (_, path) in pages(&dir) {
        let text = fs::read_to_string(&path).expect("a page is UTF-8 text");
        assert!(
            text.starts_with("# "),
            "{path}: the first line is `# title`"
        );
        assert!(
            text.lines().any(|l| l.starts_with("- `")),
            "{path}: a page has steps (`- `command` : what it does`)"
        );
        table.push_str(&format!("    include_str!({path:?}),\n"));
    }
    table.push_str("];\n");
    fs::write(out.join(format!("{folder}.rs")), table).expect("OUT_DIR is writable");
}

/// `$OUT_DIR/languages.rs` and `languages.zst`: every `languages/*.md` and
/// `languages/i18n/<language>/*.md`, each with a `name:` (English) or
/// translated text, or the build stops.
fn language_pages(out: &Path) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("languages");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut text = String::new();
    let mut table = String::from("pub static LANGUAGE_PAGES: &[(&str, (usize, usize))] = &[\n");
    for (name, path) in pages(&dir) {
        let range = add(&mut text, &path);
        assert!(
            text[range.0..range.1].starts_with("---") && text[range.0..range.1].contains("\nname:"),
            "{path}: a language page starts with its `---` header, with a `name:`"
        );
        table.push_str(&format!("    ({name:?}, {range:?}),\n"));
    }
    table.push_str("];\n");
    let i18n = dir.join("i18n");
    let mut codes: Vec<String> = fs::read_dir(&i18n)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.path().is_dir())
                .filter_map(|e| e.file_name().to_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();
    codes.sort();
    table.push_str("pub static LANGUAGE_I18N: &[(&str, &str, (usize, usize))] = &[\n");
    for code in &codes {
        let code_dir = i18n.join(code);
        println!("cargo:rerun-if-changed={}", code_dir.display());
        for (name, path) in pages(&code_dir) {
            let range = add(&mut text, &path);
            table.push_str(&format!("    ({code:?}, {name:?}, {range:?}),\n"));
        }
    }
    table.push_str("];\n");
    let packed = zstd::encode_all(text.as_bytes(), ZSTD_LEVEL).expect("zstd packs the pages");
    fs::write(out.join("languages.zst"), packed).expect("OUT_DIR is writable");
    table.push_str(concat!(
        "pub static LANGUAGE_TEXT: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| ",
        "unpack(include_bytes!(concat!(env!(\"OUT_DIR\"), \"/languages.zst\"))));\n",
    ));
    fs::write(out.join("languages.rs"), table).expect("OUT_DIR is writable");
}

/// `$OUT_DIR/i18n.rs`: every `i18n/<language>/interface.tsv` and
/// `recipes.tsv` (INTERFACE_I18N and RECIPE_I18N, (language, the file's
/// text)), each line checked: blank, a `#` comment, or `English<TAB>text`
/// with the same `{placeholders}` on both sides, or the build stops.
fn interface_i18n(out: &Path) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("i18n");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut codes: Vec<String> = fs::read_dir(&dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.path().is_dir())
                .filter_map(|e| e.file_name().to_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();
    codes.sort();
    let mut table = String::new();
    for (file, table_name) in [
        ("interface.tsv", "INTERFACE_I18N"),
        ("recipes.tsv", "RECIPE_I18N"),
    ] {
        table.push_str(&format!("pub static {table_name}: &[(&str, &str)] = &[\n"));
        for code in &codes {
            let path = dir.join(code).join(file);
            let Ok(text) = fs::read_to_string(&path) else {
                continue; // a language may have one file and not the other
            };
            for (i, line) in text.lines().enumerate() {
                if line.trim().is_empty() || line.starts_with('#') {
                    continue;
                }
                let at = format!("{}:{}", path.display(), i + 1);
                let parts: Vec<&str> = line.split('\t').collect();
                assert!(
                    parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty(),
                    "{at}: a line is `English<TAB>translation`"
                );
                assert_eq!(
                    placeholders(parts[0]),
                    placeholders(parts[1]),
                    "{at}: the translation keeps the English text's {{placeholders}}"
                );
            }
            let path = path.display().to_string();
            table.push_str(&format!("    ({code:?}, include_str!({path:?})),\n"));
        }
        table.push_str("];\n");
    }
    fs::write(out.join("i18n.rs"), table).expect("OUT_DIR is writable");
}

/// The `{name}`s in a text, sorted: a translation must have the same ones.
fn placeholders(text: &str) -> Vec<&str> {
    let mut found: Vec<&str> = text
        .match_indices('{')
        .filter_map(|(i, _)| text[i..].find('}').map(|end| &text[i..=i + end]))
        .collect();
    found.sort_unstable();
    found
}

/// The pages' folder: `content/` in a checkout, else (the crate on
/// crates.io, which takes at most 30,000 files) `content.zst` unpacked into
/// `OUT_DIR`. `scripts/release/crate.sh` packs it: records of a 0x1E
/// character, the path under content/, a newline, the page.
fn content_dir() -> std::path::PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dir = root.join("content");
    if dir.is_dir() {
        return dir;
    }
    let packed = root.join("content.zst");
    println!("cargo:rerun-if-changed={}", packed.display());
    let file = fs::File::open(&packed).expect("content/ or content.zst is there");
    let text = String::from_utf8(zstd::decode_all(file).expect("content.zst is zstd"))
        .expect("the pages are UTF-8 text");
    let out = Path::new(&env::var("OUT_DIR").expect("cargo sets OUT_DIR")).join("content");
    for record in text.split('\u{1e}').skip(1) {
        let (path, page) = record
            .split_once('\n')
            .expect("a record is path, newline, page");
        let path = out.join(path);
        fs::create_dir_all(path.parent().expect("a page is in a folder"))
            .expect("OUT_DIR is writable");
        fs::write(path, page).expect("OUT_DIR is writable");
    }
    out
}

fn main() {
    let dir = content_dir();
    // cargo watches every file under a folder named here: a page added,
    // removed or edited rebuilds the tables
    println!("cargo:rerun-if-changed={}", dir.display());
    let i18n = dir.join("i18n");
    println!("cargo:rerun-if-changed={}", i18n.display());

    let mut text = String::new();
    let mut table = String::from("pub static EMBEDDED: &[(&str, (usize, usize))] = &[\n");
    for (name, path) in pages(&dir) {
        let range = add(&mut text, &path);
        table.push_str(&format!("    ({name:?}, {range:?}),\n"));
    }
    table.push_str("];\n");

    let mut languages: Vec<String> = fs::read_dir(&i18n)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.path().is_dir())
                .filter_map(|e| e.file_name().to_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();
    languages.sort();
    table.push_str("pub static EMBEDDED_I18N: &[(&str, &str, (usize, usize))] = &[\n");
    for lang in &languages {
        let lang_dir = i18n.join(lang);
        println!("cargo:rerun-if-changed={}", lang_dir.display());
        for (name, path) in pages(&lang_dir) {
            let range = add(&mut text, &path);
            table.push_str(&format!("    ({lang:?}, {name:?}, {range:?}),\n"));
        }
    }
    table.push_str("];\n");

    let out = Path::new(&env::var("OUT_DIR").expect("cargo sets OUT_DIR")).to_path_buf();
    let packed = zstd::encode_all(text.as_bytes(), ZSTD_LEVEL).expect("zstd packs the pages");
    fs::write(out.join("pages.zst"), packed).expect("OUT_DIR is writable");
    // unpacked once, the first time a page is needed (the list needs them at
    // start, for subcommands and categories)
    table.push_str(concat!(
        "pub static EMBEDDED_TEXT: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| ",
        "unpack(include_bytes!(concat!(env!(\"OUT_DIR\"), \"/pages.zst\"))));\n",
    ));
    fs::write(out.join("content.rs"), table).expect("OUT_DIR is writable");
    palettes(&out);
    recipes(&out);
    language_pages(&out);
    interface_i18n(&out);
}
