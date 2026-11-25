//! Hardcoded values: the interface in the reader's language.

use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;

/// Whether the screen's texts follow Pages in (the about screen's setting,
/// else `LANG`). Only the full-screen app turns it on, so what lore prints
/// for scripts and agents (`lore -p`, `search`, `explain`, `--help`, the
/// errors) stays in English, the same on every machine.
pub static TRANSLATE_INTERFACE: AtomicBool = AtomicBool::new(false);

/// The translation files, one folder per language in the repository
/// (`i18n/pt_BR/`), built in by build.rs: the screen's texts, and the
/// recipes' names. Format: i18n/README.md.
pub const I18N_INTERFACE_FILE: &str = "interface.tsv";
pub const I18N_RECIPES_FILE: &str = "recipes.tsv";
/// Those files read, once, the first time a text is looked up:
/// (language, English → translated).
pub static INTERFACE_CACHE: OnceLock<Vec<(&str, HashMap<String, String>)>> = OnceLock::new();
pub static RECIPE_CACHE: OnceLock<Vec<(&str, HashMap<String, String>)>> = OnceLock::new();

/// After a name that hides another entry of the same name, and what each
/// kind of entry is called there.
pub const HIDES: &str = "  (hides {what})";
pub const HIDES_ALIAS: &str = "another alias";
pub const HIDES_KEYWORD: &str = "a keyword";
pub const HIDES_FUNCTION: &str = "a function";
pub const HIDES_BUILTIN: &str = "a builtin";
pub const HIDES_PROGRAM: &str = "a program";
pub const HIDES_SUBCOMMAND: &str = "a subcommand";
pub const HIDES_RECIPE: &str = "a recipe";
pub const HIDES_TOOL: &str = "a tool page";
pub const HIDES_LANGUAGE: &str = "a language";
pub const HIDES_DOCS: &str = "documentation";
