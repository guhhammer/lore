//! Hardcoded values: screen.

use ratatui::style::Modifier;

/// The list's share of the width, in percent, while a page is open; the page
/// gets the rest.
pub const INFO_LIST_PERCENT: u16 = 10;

/// Lines that PgUp and PgDn scroll the page by.
pub const PAGE_STEP: isize = 20;

/// The list's indents: of a command, of a letter header; beside a page,
/// where a tenth of the width is all there is, less.
pub const LIST_INDENT: &str = "   ";
pub const LIST_HEADER_INDENT: &str = " ";
pub const LIST_INDENT_NARROW: &str = " ";

/// The header of names that start with a digit or a symbol.
pub const SYMBOL_SECTION: &str = "#";

/// Lines the bordered search bar takes; the key bar takes one.
pub const SEARCH_BAR_HEIGHT: u16 = 3;
pub const KEY_BAR_HEIGHT: u16 = 1;
/// The key bar wraps onto a second row when one is too narrow, never more.
pub const KEY_BAR_ROWS_MAX: usize = 2;
/// Columns between two keys on the key bar.
pub const KEY_GAP: &str = "   ";

/// The smallest window lore draws in; below it, a note says what it needs.
pub const MIN_WIDTH: u16 = 50;
pub const MIN_HEIGHT: u16 = 14;
pub const TOO_SMALL: &str = "This window is {width}×{height}.\nlore needs at least {min_width}×{min_height}:\nmake it bigger.";

/// The ending that marks text cut short.
pub const ELLIPSIS: &str = "...";

/// Bold: headings, letter headers, `danger`.
pub const MODIFIER_HEADING: Modifier = Modifier::BOLD;
pub const MODIFIER_SECTION: Modifier = Modifier::BOLD;
pub const MODIFIER_DANGER: Modifier = Modifier::BOLD;
