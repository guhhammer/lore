//! What the page beside the list shows for a command: lore's own page for
//! it, its package and version, how to use it, links, then its manual.

pub mod content; // the written pages: built in, or from CONTENT_DIRS
pub mod danger; // what can destroy data or stop the system
pub mod links; // the page's links and the web searches
pub mod package; // package, version and updates, from the package manager
pub mod page; // all of it, as lines to draw
