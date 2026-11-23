//! Puts the default pages of `content/` inside the binary, so lore has them
//! wherever it runs, offline, however it was installed (a package, `cargo
//! install`, a copied file). Every page goes into one zstd-compressed text,
//! `$OUT_DIR/pages.zst` (19 MB of Markdown packs to about 3 MB), and
//! `$OUT_DIR/content.rs` holds the tables: EMBEDDED, (command name, where its
//! page sits in that text), and EMBEDDED_I18N, (language, name, where) from
//! `content/i18n/<language>/`, both sorted for binary search.

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

fn main() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
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
}
