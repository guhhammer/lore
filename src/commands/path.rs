//! The programs on the machine: executable files in the `$PATH` directories,
//! the ones listed in `PATH_LIST_FILES` (macOS), and `PROGRAM_DIRS`.

use std::collections::HashSet;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use super::{Command, Kind};
use crate::compile_config::{PATH_LIST_FILES, PROGRAM_DIRS};
use crate::paths;

/// Every executable file found, unsorted; duplicates are removed later by `merge`.
pub fn scan() -> Vec<Command> {
    let mut commands = Vec::new();
    for dir in dirs() {
        let Ok(entries) = fs::read_dir(dir) else {
            continue; // missing or unreadable directory
        };
        for entry in entries.flatten() {
            // fs::metadata follows symlinks; DirEntry::metadata would not, and most
            // commands (python3, vi, everything in /etc/alternatives) are symlinks.
            let Ok(meta) = fs::metadata(entry.path()) else {
                continue; // broken symlink
            };
            // 0o111 = any execute bit (owner, group or others)
            if meta.is_file() && meta.permissions().mode() & 0o111 != 0 {
                commands.push(Command::new(
                    entry.file_name().to_string_lossy(),
                    Kind::Program,
                ));
            }
        }
    }
    commands
}

/// The directories to scan: `$PATH`'s, the listed ones, then `PROGRAM_DIRS`,
/// each real directory once (on Debian /bin is a link to /usr/bin).
fn dirs() -> Vec<PathBuf> {
    let home = paths::home();
    let path = env::var_os("PATH").unwrap_or_default();
    // macOS: /etc/paths and /etc/paths.d/* hold one directory per line
    let listed: Vec<PathBuf> = PATH_LIST_FILES
        .iter()
        .flat_map(|pattern| paths::expand(pattern, home.as_deref()))
        .filter_map(|file| paths::read_lossy(&file))
        .flat_map(|text| {
            text.lines()
                .map(str::trim)
                .filter(|line| !line.is_empty() && !line.starts_with('#'))
                .map(PathBuf::from)
                .collect::<Vec<_>>()
        })
        .collect();
    let configured = PROGRAM_DIRS
        .iter()
        .flat_map(|pattern| paths::expand(pattern, home.as_deref()));
    let mut seen = HashSet::new();
    env::split_paths(&path)
        .chain(listed)
        .chain(configured)
        // canonicalize resolves links; insert returns false for one already seen
        .filter(|dir| seen.insert(fs::canonicalize(dir).unwrap_or_else(|_| dir.clone())))
        .collect()
}
