//! Small helpers the rest of the app leans on.

pub mod clipboard; // copying through the terminal (OSC 52), over ssh too
pub mod highlight; // code in colours, like bat, for the languages topic
pub mod history; // how often each command was run, from the shell's history
pub mod html; // HTML as page lines, for the docs topic
pub mod internet; // the INTERNET section: the connection and cheat.sh, in the background
pub mod manual; // a command's man page, as text
pub mod text; // cutting text to a column, with `...`
pub mod zip; // the files in a zip archive (tldr-pages' releases)
