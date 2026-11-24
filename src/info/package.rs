//! Which package a program came from, its installed version, and the newest
//! version the package manager knows of: all offline, from the package
//! manager's own records (dpkg and apt, rpm, pacman, Homebrew's folders).
//! Nothing is installed or updated; lore only says how to.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::configs::{UPDATE_DPKG, UPDATE_HOMEBREW, UPDATE_PACMAN, UPDATE_RPM};

/// What lore could learn about a program's package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    /// The package manager: "dpkg", "rpm", "pacman", "Homebrew".
    pub manager: &'static str,
    pub name: String,
    pub installed: Option<String>,
    /// The newest version known (apt's candidate); None when lore cannot tell.
    pub newest: Option<String>,
    /// The command that checks for, or installs, updates.
    pub update: String,
}

/// The package that installed the program at `path`, if a package manager
/// lore knows claims it. Tries each manager; one that is missing is skipped.
pub fn of(path: &Path) -> Option<Package> {
    // the link (/usr/bin/vi) and what it points to (/usr/bin/vim.basic)
    // can belong to different packages; the real file is the program
    let real = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let candidates: Vec<PathBuf> = if real == path {
        vec![real]
    } else {
        vec![real, path.to_path_buf()]
    };
    candidates.iter().find_map(|file| {
        homebrew(file)
            .or_else(|| dpkg(file))
            .or_else(|| rpm(file))
            .or_else(|| pacman(file))
    })
}

/// `program args…`'s output, or None if it is missing or fails.
fn run(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program)
        .args(args)
        .env("LC_ALL", "C") // English, so the output can be read
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

fn dpkg(file: &Path) -> Option<Package> {
    let owner = run("dpkg-query", &["-S", file.to_str()?])?;
    let name = dpkg_owner(&owner)?;
    let installed = run("dpkg-query", &["-W", "-f=${Version}", &name])
        .map(|v| v.trim().to_owned())
        .filter(|v| !v.is_empty());
    let newest = run("apt-cache", &["policy", &name]).and_then(|p| apt_candidate(&p));
    Some(Package {
        manager: "dpkg",
        update: UPDATE_DPKG.replace("{name}", &name),
        name,
        installed,
        newest,
    })
}

/// The package in `dpkg-query -S`'s answer ("coreutils: /usr/bin/ls"),
/// skipping diversion lines; the first when several share the file.
pub fn dpkg_owner(output: &str) -> Option<String> {
    output
        .lines()
        .filter(|line| !line.starts_with("diversion"))
        .find_map(|line| line.split_once(": "))
        .map(|(packages, _)| {
            packages
                .split(", ")
                .next()
                .unwrap_or(packages)
                .trim()
                .to_owned()
        })
        .filter(|name| !name.is_empty())
}

/// The "Candidate:" of `apt-cache policy`, unless it is "(none)".
pub fn apt_candidate(policy: &str) -> Option<String> {
    policy
        .lines()
        .find_map(|line| line.trim().strip_prefix("Candidate:"))
        .map(|v| v.trim().to_owned())
        .filter(|v| !v.is_empty() && v != "(none)")
}

fn rpm(file: &Path) -> Option<Package> {
    let found = run(
        "rpm",
        &[
            "-qf",
            "--qf",
            "%{NAME} %{VERSION}-%{RELEASE}\\n",
            file.to_str()?,
        ],
    )?;
    let (name, version) = found.lines().next()?.split_once(' ')?;
    Some(Package {
        manager: "rpm",
        name: name.to_owned(),
        installed: Some(version.to_owned()),
        newest: None,
        update: UPDATE_RPM.replace("{name}", name),
    })
}

fn pacman(file: &Path) -> Option<Package> {
    let (name, version) = pacman_owner(&run("pacman", &["-Qo", file.to_str()?])?)?;
    Some(Package {
        manager: "pacman",
        update: UPDATE_PACMAN.to_owned(),
        name,
        installed: Some(version),
        newest: None,
    })
}

/// (package, version) from `pacman -Qo`: "/usr/bin/ls is owned by coreutils 9.5-1".
pub fn pacman_owner(output: &str) -> Option<(String, String)> {
    let (_, owner) = output.lines().next()?.split_once(" is owned by ")?;
    let (name, version) = owner.trim().split_once(' ')?;
    Some((name.to_owned(), version.to_owned()))
}

fn homebrew(file: &Path) -> Option<Package> {
    let (name, version) = homebrew_cellar(file)?;
    Some(Package {
        manager: "Homebrew",
        update: UPDATE_HOMEBREW.replace("{name}", &name),
        name,
        installed: Some(version),
        newest: None,
    })
}

/// (formula, version) from a path inside Homebrew's Cellar:
/// …/Cellar/<formula>/<version>/bin/<program>. Reading the path is instant;
/// running `brew` takes seconds.
pub fn homebrew_cellar(file: &Path) -> Option<(String, String)> {
    let parts: Vec<&str> = file.iter().filter_map(|p| p.to_str()).collect();
    let cellar = parts.iter().position(|p| *p == "Cellar")?;
    let name = parts.get(cellar + 1)?;
    let version = parts.get(cellar + 2)?;
    Some(((*name).to_owned(), (*version).to_owned()))
}
