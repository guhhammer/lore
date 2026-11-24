//! Hardcoded values: content pages.

/// Optional folders of extra pages (`<command>.md`, translations in
/// `i18n/<lang>/<command>.md`). None has to exist: lore carries its own
/// pages. Every page found in them must pass `info::check`, or it is not
/// shown.
///
/// Yours come first and may replace a page built into lore.
pub const USER_CONTENT_DIRS: &[&str] = &["~/.local/share/lore/content"];
/// Where other programs install a page for their command (a package's
/// `/usr/share/lore/content/NAME.md`): used only for a command lore has no
/// page of its own for, so no package can rewrite another command's page.
pub const SYSTEM_CONTENT_DIRS: &[&str] =
    &["/usr/local/share/lore/content", "/usr/share/lore/content"];

/// Pages `lore --update-pages` downloaded: used only for a command with no
/// page of yours or lore's own, like `SYSTEM_CONTENT_DIRS`. The first one
/// is where they are written; lore replaces the whole folder each time.
pub const UPDATED_CONTENT_DIRS: &[&str] = &["~/.local/share/lore/updates"];
