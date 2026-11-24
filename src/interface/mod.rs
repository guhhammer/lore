//! What the user sees and does: the keys, the list's rows and sections, the
//! search, the drawing, and the paths the configured lists are written in.

pub mod args; // the command line: `lore`, `lore NAME`, --help, --version
pub mod input; // keys and resizes → Action
pub mod palette; // the colours, chosen by $LORE_PALETTE
pub mod paths; // `~/` and `*` in the configured paths
pub mod rows; // commands grouped under section headers
pub mod search; // what matches the search text
pub mod section; // which letter a name goes under

pub(crate) mod ui; // drawing; only run() needs it
