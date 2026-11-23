//! The user's shell, and its builtins and keywords (lists in compile_config.rs).

use std::env;

use super::{Command, Kind};
use crate::compile_config as config;

/// The shells lore knows. Any other (sh, dash, ksh…) is treated as bash, the
/// closest in syntax and builtins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Shell {
    /// The login shell, from `$SHELL` (`/bin/zsh` → Zsh); bash if unset.
    pub fn current() -> Self {
        env::var("SHELL").map_or(Self::Bash, |path| Self::from_path(&path))
    }

    /// The shell a path or name stands for, by its last component.
    pub fn from_path(path: &str) -> Self {
        match path.rsplit('/').next().unwrap_or(path) {
            "zsh" => Self::Zsh,
            "fish" => Self::Fish,
            _ => Self::Bash,
        }
    }

    /// The files its aliases are defined in, as patterns, system-wide first.
    pub fn alias_files(self) -> &'static [&'static str] {
        match self {
            Self::Bash => config::BASH_ALIAS_FILES,
            Self::Zsh => config::ZSH_ALIAS_FILES,
            Self::Fish => config::FISH_ALIAS_FILES,
        }
    }

    /// `cd`, `export`, `echo`…: commands the shell runs itself.
    pub fn builtins(self) -> Vec<Command> {
        let names = match self {
            Self::Bash => config::BASH_BUILTINS,
            Self::Zsh => config::ZSH_BUILTINS,
            Self::Fish => config::FISH_BUILTINS,
        };
        commands(names, Kind::Builtin)
    }

    /// `if`, `for`, `[[`…: the shell's grammar words.
    pub fn keywords(self) -> Vec<Command> {
        let names = match self {
            Self::Bash => config::BASH_KEYWORDS,
            Self::Zsh => config::ZSH_KEYWORDS,
            Self::Fish => config::FISH_KEYWORDS,
        };
        commands(names, Kind::Keyword)
    }
}

/// Each name as a command of `kind`.
fn commands(names: &[&str], kind: Kind) -> Vec<Command> {
    names.iter().map(|&n| Command::new(n, kind)).collect()
}
