//! Small helpers the rest of the app leans on.

pub mod clipboard; // copying through the terminal (OSC 52), over ssh too
pub mod history;
pub mod internet; // the INTERNET section: the connection and cheat.sh, in the background // how often each command was run, from the shell's history
pub mod manual; // a command's man page, as text
pub mod text; // cutting text to a column, with `...`
