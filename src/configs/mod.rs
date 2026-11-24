//! Hardcoded values, fixed at compile time. To change one, edit it in this folder and
//! rebuild (`cli/make`).
//!
//! Every path here is optional: a missing file or directory is skipped, never
//! an error, so one list serves Linux and macOS users of bash, zsh and fish.
//! Path syntax: `~/` is the user's home (skipped if `$HOME` is unset), and a
//! `*` stands for any name within one path component (`/opt/*/bin`).
//!
//! One file per part of lore; every value is reachable as
//! `crate::configs::NAME`, whichever file holds it.

mod beacon;
mod check;
mod completion;
mod content;
mod danger;
mod docs;
mod explain;
mod internet;
mod languages;
mod logo;
mod man;
mod own_use;
mod packages;
mod page;
mod programs;
mod screen;
mod search;
mod settings;
mod shells;
mod texts;
mod updates;

pub use beacon::*;
pub use check::*;
pub use completion::*;
pub use content::*;
pub use danger::*;
pub use docs::*;
pub use explain::*;
pub use internet::*;
pub use languages::*;
pub use logo::*;
pub use man::*;
pub use own_use::*;
pub use packages::*;
pub use page::*;
pub use programs::*;
pub use screen::*;
pub use search::*;
pub use settings::*;
pub use shells::*;
pub use texts::*;
pub use updates::*;
