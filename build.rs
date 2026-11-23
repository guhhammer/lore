//! Puts the default pages of `content/` inside the binary, so lore has them
//! wherever it runs, offline, however it was installed (a package, `cargo
//! install`, a copied file). Writes `$OUT_DIR/content.rs`: EMBEDDED, a table
//! of (command name, page text), and EMBEDDED_I18N, (language, name, text)
//! from `content/i18n/<language>/`, both sorted for binary search.

use std::env;
use std::fs;
use std::path::Path;

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

    let mut table = String::from("pub static EMBEDDED: &[(&str, &str)] = &[\n");
    for (name, path) in pages(&dir) {
        table.push_str(&format!("    ({name:?}, include_str!({path:?})),\n"));
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
    table.push_str("pub static EMBEDDED_I18N: &[(&str, &str, &str)] = &[\n");
    for lang in &languages {
        let lang_dir = i18n.join(lang);
        println!("cargo:rerun-if-changed={}", lang_dir.display());
        for (name, path) in pages(&lang_dir) {
            table.push_str(&format!(
                "    ({lang:?}, {name:?}, include_str!({path:?})),\n"
            ));
        }
    }
    table.push_str("];\n");

    let out = Path::new(&env::var("OUT_DIR").expect("cargo sets OUT_DIR")).join("content.rs");
    fs::write(out, table).expect("OUT_DIR is writable");
}
