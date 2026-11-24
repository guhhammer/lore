//! What the page beside the list shows for a command: lore's own page for
//! it, its package and version, how to use it, links, then its manual.

pub mod beacon; // what people use and install most, and what is rising
pub mod check; // the check of pages other programs ship: no code, nothing hidden
pub mod content; // the written pages: built in, or from the content folders
pub mod danger; // what can destroy data or stop the system
pub mod docs; // the docs topic: DevDocs sets, downloaded when asked
pub mod explain; // `lore explain "tar -xzvf a.tgz"`: each word, from the man page
pub mod languages; // learnxinyminutes.com's pages, shown like bat shows a file
pub mod links; // the page's links and the web searches
pub mod package; // package, version and updates, from the package manager
pub mod page; // all of it, as lines to draw
pub mod recipes; // task pages that chain commands, from recipes/
pub mod tldr; // a tldr-pages page in lore's format, for --update-pages
pub mod tools; // cheat sheets of a tool's everyday tasks, from tools/
pub mod updates; // `lore --update-pages`: newer pages, only when asked
