//! Hardcoded values: programs.

/// Always scanned for programs, besides `$PATH`, which is often cut down
/// (desktop launchers, `sudo`, fresh logins). A normal Debian user's `$PATH`
/// has no sbin, which holds the commands for running the machine (reboot,
/// fdisk, useradd…): exactly what is needed offline.
#[rustfmt::skip]
pub const PROGRAM_DIRS: &[&str] = &[
    // Linux and macOS
    "/usr/local/sbin", "/usr/local/bin", "/usr/sbin", "/usr/bin", "/sbin", "/bin",
    "/usr/games", "/usr/local/games",
    // Snap, Flatpak (one command per installed app)
    "/snap/bin", "/var/lib/flatpak/exports/bin", "~/.local/share/flatpak/exports/bin",
    // Nix and NixOS
    "~/.nix-profile/bin", "/nix/var/nix/profiles/default/bin", "/run/current-system/sw/bin",
    // Homebrew: Linux, macOS on Apple silicon (Intel Macs use /usr/local/bin)
    "/home/linuxbrew/.linuxbrew/bin", "/home/linuxbrew/.linuxbrew/sbin",
    "/opt/homebrew/bin", "/opt/homebrew/sbin",
    // MacPorts, and macOS's own extra system directories
    "/opt/local/bin", "/opt/local/sbin",
    "/System/Cryptexes/App/usr/bin", "/Library/Apple/usr/bin",
    // the user's own and language tools
    "~/bin", "~/.local/bin", "~/.cargo/bin", "~/go/bin",
    // software installed under /opt
    "/opt/*/bin",
];

/// Files listing one program directory per line. macOS builds its default
/// `$PATH` from these; on Linux they usually do not exist.
pub const PATH_LIST_FILES: &[&str] = &["/etc/paths", "/etc/paths.d/*"];
