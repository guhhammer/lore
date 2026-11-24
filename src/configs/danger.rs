//! Hardcoded values: danger.

/// Examples marked dangerous on a page: (program, what in the rest of the
/// line makes it so; "" = always). A program ending in `*` is a prefix
/// (`mkfs*` covers mkfs.ext4). A leading `sudo` is skipped. Whole commands
/// are marked by their page's `danger` category instead.
#[rustfmt::skip]
pub const DANGEROUS_EXAMPLES: &[(&str, &str)] = &[
    // delete or overwrite, with no undo
    ("rm", ""), ("shred", ""), ("wipefs", ""), ("srm", ""), ("truncate", ""),
    ("dd", "of="), ("mkfs*", ""), ("mke2fs", ""), ("mkswap", ""),
    ("fdisk", ""), ("sfdisk", ""), ("sgdisk", ""), ("gdisk", ""), ("parted", ""),
    ("find", "-delete"), ("find", "-exec rm"), ("rsync", "--delete"),
    ("git", "reset --hard"), ("git", "clean -f"), ("git", "push --force"), ("git", "push -f"),
    ("git", "branch -D"), ("git", "checkout -- "), ("git", "restore "),
    ("chmod", "-R"), ("chown", "-R"), ("crontab", "-r"), ("userdel", ""), ("deluser", ""),
    ("docker", "prune"), ("docker", " rm"), ("kubectl", "delete"), ("terraform", "destroy"),
    ("iptables", "-F"), ("mv", "-f"), ("cp", "-f"),
    // stop the machine or processes
    ("reboot", ""), ("shutdown", ""), ("poweroff", ""), ("halt", ""),
    ("kill", "-9"), ("killall", ""), ("pkill", ""),
    // Windows
    ("format", ""), ("diskpart", ""), ("del", ""), ("rd", "/s"), ("rmdir", "/s"),
];

/// Words before a command that run it as another user: skipped when
/// judging an example (`sudo rm …` is judged as `rm …`).
pub const RUN_AS: [&str; 2] = ["sudo", "doas"];

/// Anywhere in an example, these make it dangerous whatever the program:
/// writing straight onto a disk device.
pub const DANGEROUS_ANYWHERE: &[&str] = &["of=/dev/sd", "of=/dev/nvme", "> /dev/sd", "> /dev/nvme"];
