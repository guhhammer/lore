//! Hardcoded values, fixed at compile time. To change one, edit it here and
//! rebuild (`cli/make`).
//!
//! Every path here is optional: a missing file or directory is skipped, never
//! an error, so one list serves Linux and macOS users of bash, zsh and fish.
//! Path syntax: `~/` is the user's home (skipped if `$HOME` is unset), and a
//! `*` stands for any name within one path component (`/opt/*/bin`).

// ── programs ────────────────────────────────────────────────────────────────

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

// ── bash ────────────────────────────────────────────────────────────────────

/// Where bash aliases are defined: system-wide first, then the user's, so the
/// user's win (a later definition wins). `/etc/bashrc` is the name on macOS
/// and Fedora, `/etc/bash.bashrc` on Debian.
#[rustfmt::skip]
pub const BASH_ALIAS_FILES: &[&str] = &[
    "/etc/profile", "/etc/bash.bashrc", "/etc/bashrc", "/etc/profile.d/*.sh",
    "~/.profile", "~/.bash_profile", "~/.bash_login", "~/.bash_aliases", "~/.bashrc",
];

/// Bash 5.2's builtins (`compgen -b`). They are not files, so no directory
/// has them; listing them here avoids starting a bash just to ask.
#[rustfmt::skip]
pub const BASH_BUILTINS: &[&str] = &[
    ".", ":", "[", "alias", "bg", "bind", "break", "builtin", "caller", "cd", "command",
    "compgen", "complete", "compopt", "continue", "declare", "dirs", "disown", "echo", "enable",
    "eval", "exec", "exit", "export", "false", "fc", "fg", "getopts", "hash", "help", "history",
    "jobs", "kill", "let", "local", "logout", "mapfile", "popd", "printf", "pushd", "pwd", "read",
    "readarray", "readonly", "return", "set", "shift", "shopt", "source", "suspend", "test",
    "times", "trap", "true", "type", "typeset", "ulimit", "umask", "unalias", "unset", "wait",
];

/// Bash 5.2's keywords (`compgen -k`).
#[rustfmt::skip]
pub const BASH_KEYWORDS: &[&str] = &[
    "if", "then", "else", "elif", "fi", "case", "esac", "for", "select", "while", "until", "do",
    "done", "in", "function", "time", "{", "}", "!", "[[", "]]", "coproc",
];

// ── zsh (the default shell on macOS) ────────────────────────────────────────

/// Where zsh aliases are defined, system-wide first. `/etc/zsh/` is Debian's
/// layout, `/etc/` macOS's. oh-my-zsh: only the user's custom files; its
/// plugins' aliases depend on the `plugins=(…)` line and are not read.
#[rustfmt::skip]
pub const ZSH_ALIAS_FILES: &[&str] = &[
    "/etc/zshenv", "/etc/zsh/zshenv", "/etc/zprofile", "/etc/zsh/zprofile",
    "/etc/zshrc", "/etc/zsh/zshrc",
    "~/.zshenv", "~/.zprofile", "~/.zshrc", "~/.zsh_aliases", "~/.oh-my-zsh/custom/*.zsh",
];

/// Zsh 5.9's builtins, from the zshbuiltins manual, including those of its
/// standard modules (compctl, zle, zstyle…), which most setups load.
#[rustfmt::skip]
pub const ZSH_BUILTINS: &[&str] = &[
    "-", ".", ":", "[", "alias", "autoload", "bg", "bindkey", "break", "builtin", "bye", "cap",
    "cd", "chdir", "clone", "command", "comparguments", "compcall", "compctl", "compdescribe",
    "compfiles", "compgroups", "compquote", "comptags", "comptry", "compvalues", "continue",
    "declare", "dirs", "disable", "disown", "echo", "echotc", "echoti", "emulate", "enable",
    "eval", "exec", "exit", "export", "false", "fc", "fg", "float", "functions", "getcap",
    "getln", "getopts", "hash", "history", "integer", "jobs", "kill", "let", "limit", "local",
    "logout", "noglob", "popd", "print", "printf", "pushd", "pushln", "pwd", "r", "read",
    "readonly", "rehash", "return", "sched", "set", "setcap", "setopt", "shift", "source",
    "stat", "suspend", "test", "times", "trap", "true", "ttyctl", "type", "typeset", "ulimit",
    "umask", "unalias", "unfunction", "unhash", "unlimit", "unset", "unsetopt", "vared", "wait",
    "whence", "where", "which", "zcompile", "zformat", "zftp", "zle", "zmodload", "zparseopts",
    "zprof", "zpty", "zregexparse", "zsocket", "zstyle", "ztcp",
];

/// Zsh 5.9's reserved words (the zshmisc manual, "Reserved Words").
#[rustfmt::skip]
pub const ZSH_KEYWORDS: &[&str] = &[
    "do", "done", "esac", "then", "elif", "else", "fi", "for", "case", "if", "while", "function",
    "repeat", "time", "until", "select", "coproc", "nocorrect", "foreach", "end", "!", "[[", "]]",
    "{", "}", "declare", "export", "float", "integer", "local", "readonly", "typeset",
];

// ── fish ────────────────────────────────────────────────────────────────────

/// Where fish aliases (`alias name 'value'`) are defined, system-wide first.
#[rustfmt::skip]
pub const FISH_ALIAS_FILES: &[&str] = &[
    "/etc/fish/config.fish", "/etc/fish/conf.d/*.fish",
    "~/.config/fish/config.fish", "~/.config/fish/conf.d/*.fish",
];

/// Fish functions: one per file, named like the file (`mkcd.fish` → `mkcd`).
pub const FISH_FUNCTION_FILES: &[&str] = &["~/.config/fish/functions/*.fish"];

/// Fish's builtins. The fish manual does not mark which commands are
/// builtins, so this is fish 3's `builtin -n` from memory, minus the keywords
/// below: worth checking against `builtin -n` on a machine with fish.
#[rustfmt::skip]
pub const FISH_BUILTINS: &[&str] = &[
    ".", ":", "[", "_", "abbr", "argparse", "bg", "bind", "block", "breakpoint", "cd",
    "commandline", "complete", "contains", "count", "disown", "echo", "emit", "eval", "exit",
    "false", "fg", "functions", "history", "jobs", "math", "path", "printf", "pwd", "random",
    "read", "realpath", "set", "set_color", "source", "status", "string", "test", "true", "type",
    "ulimit", "wait",
];

/// Fish's keywords (the fish manual, "Keywords").
#[rustfmt::skip]
pub const FISH_KEYWORDS: &[&str] = &[
    "if", "else", "for", "while", "break", "continue", "function", "return", "begin", "end",
    "and", "or", "not", "switch", "case", "command", "builtin", "time", "exec",
];
