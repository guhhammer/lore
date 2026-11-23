//! The user's shell functions. Only fish keeps them in a way that can be read
//! reliably (one file per function); bash and zsh functions are not listed.

use super::shell::Shell;
use super::{Command, Kind};
use crate::compile_config::FISH_FUNCTION_FILES;
use crate::interface::paths;

/// The user's functions for `shell`; none for bash and zsh.
pub fn scan(shell: Shell) -> Vec<Command> {
    if shell != Shell::Fish {
        return Vec::new();
    }
    let home = paths::home();
    FISH_FUNCTION_FILES
        .iter()
        .flat_map(|pattern| paths::expand(pattern, home.as_deref()))
        // the function is named like its file: mkcd.fish → mkcd
        .filter_map(|file| file.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .map(|name| Command::new(name, Kind::Function))
        .collect()
}
