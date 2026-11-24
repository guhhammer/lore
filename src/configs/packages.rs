//! Hardcoded values: packages.

/// How each package manager updates a package (`{name}`), shown on its page.
pub const UPDATE_DPKG: &str = "sudo apt update && sudo apt install --only-upgrade {name}";
pub const UPDATE_RPM: &str = "sudo dnf upgrade {name}";
/// Arch upgrades as a whole, never one package.
pub const UPDATE_PACMAN: &str = "sudo pacman -Syu";
pub const UPDATE_HOMEBREW: &str = "brew update && brew upgrade {name}";
