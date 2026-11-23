"""The categories lore shows after each name: the keywords that place a page
in one, the categories set by hand for the commands people meet first, and
the commands that can destroy data or stop the system (`danger`)."""

# category → words looked for in a page's name, summary and examples
KEYWORDS = {
    "files": ["file", "directory", "directories", "folder", "path", "symlink", "link", "copy", "move", "rename", "permission"],
    "text": ["text", "line", "lines", "string", "pattern", "regex", "regular expression", "word", "column", "csv", "json", "yaml", "xml", "character"],
    "archives": ["archive", "compress", "decompress", "extract", "zip", "tar", "gzip", "bzip", "xz", "zstd", "7z", "rar"],
    "network": ["network", "http", "https", "url", "host", "port", "dns", "ip address", "download", "upload", "ssh", "ftp", "socket", "wifi", "wi-fi", "proxy", "ping", "packet", "interface"],
    "processes": ["process", "processes", "pid", "signal", "job", "jobs", "priority", "thread"],
    "system": ["service", "services", "systemd", "boot", "kernel", "daemon", "system log", "journal", "module", "cron", "schedule", "shutdown", "reboot", "locale", "time zone", "timezone", "clock"],
    "disks": ["disk", "disks", "partition", "filesystem", "file system", "mount", "block device", "volume", "raid", "lvm", "swap", "drive"],
    "users": ["user", "users", "group", "groups", "password", "login", "account", "sudo"],
    "packages": ["package", "packages", "install", "repository", "dependencies", "upgrade"],
    "development": ["compile", "compiler", "build", "debug", "debugger", "library", "source code", "programming", "interpreter", "script", "linker", "test", "tests", "project", "module"],
    "version-control": ["git", "commit", "branch", "merge", "svn", "mercurial", "version control", "diff", "patch"],
    "containers": ["container", "containers", "docker", "image", "images", "kubernetes", "pod", "pods", "podman", "helm", "cluster"],
    "cloud": ["cloud", "aws", "azure", "google cloud", "gcloud", "bucket", "s3", "serverless", "heroku", "vercel", "netlify"],
    "security": ["encrypt", "decrypt", "certificate", "gpg", "pgp", "ssl", "tls", "hash", "checksum", "firewall", "key pair", "signature", "secure", "vulnerab", "password manager"],
    "media": ["image", "images", "video", "audio", "sound", "music", "picture", "photo", "screenshot", "record", "stream", "pdf", "font", "color", "colour"],
    "shell": ["shell", "terminal", "environment variable", "alias", "prompt", "history", "builtin", "command line", "tty"],
    "editors": ["editor", "edit a file", "text editor", "vim", "emacs", "nano"],
    "monitoring": ["monitor", "usage", "statistics", "performance", "benchmark", "load", "uptime", "memory", "cpu usage", "log"],
    "hardware": ["usb", "pci", "cpu", "battery", "bluetooth", "device", "devices", "sensor", "firmware", "bios", "gpu", "printer", "keyboard", "mouse", "monitor brightness"],
    "databases": ["database", "sql", "query", "table", "postgres", "mysql", "sqlite", "redis", "mongo"],
    "desktop": ["window", "clipboard", "notification", "display", "screen", "desktop", "x11", "wayland", "gnome", "kde", "xdg"],
    "documents": ["document", "latex", "markdown", "pdf", "print", "manual page", "man page", "documentation", "ebook", "office"],
}

# set by hand; wins over the keywords
HAND = {
    "ls": "files", "cd": "files, shell", "cp": "files", "mv": "files", "rm": "files", "mkdir": "files",
    "rmdir": "files", "touch": "files", "ln": "files", "find": "files", "locate": "files", "tree": "files",
    "chmod": "files, users", "chown": "files, users", "stat": "files", "file": "files", "du": "disks, files",
    "df": "disks", "mount": "disks", "umount": "disks", "fdisk": "disks", "parted": "disks", "lsblk": "disks, hardware",
    "mkfs": "disks", "dd": "disks, files", "grep": "text", "sed": "text", "awk": "text", "cut": "text", "sort": "text",
    "uniq": "text", "wc": "text", "tr": "text", "head": "text, files", "tail": "text, files", "cat": "text, files",
    "less": "text, files", "more": "text, files", "diff": "text, version-control", "jq": "text", "xargs": "shell",
    "tar": "archives", "zip": "archives", "unzip": "archives", "gzip": "archives", "xz": "archives", "7z": "archives",
    "ssh": "network, security", "scp": "network, files", "rsync": "network, files", "curl": "network", "wget": "network",
    "ping": "network", "ip": "network", "ss": "network", "netstat": "network", "dig": "network", "nslookup": "network",
    "nc": "network", "nmap": "network, security", "traceroute": "network", "ps": "processes", "top": "processes, monitoring",
    "htop": "processes, monitoring", "kill": "processes", "killall": "processes", "pkill": "processes", "nohup": "processes",
    "systemctl": "system", "journalctl": "system, monitoring", "reboot": "system", "shutdown": "system", "crontab": "system",
    "sudo": "users, system", "su": "users", "passwd": "users, security", "useradd": "users", "usermod": "users",
    "userdel": "users", "groups": "users", "id": "users", "whoami": "users", "apt": "packages", "apt-get": "packages",
    "dpkg": "packages", "dnf": "packages", "yum": "packages", "pacman": "packages", "brew": "packages", "snap": "packages",
    "flatpak": "packages", "pip": "packages, development", "npm": "packages, development", "cargo": "packages, development",
    "git": "version-control", "docker": "containers", "podman": "containers", "kubectl": "containers", "helm": "containers",
    "make": "development", "gcc": "development", "python": "development", "python3": "development", "node": "development",
    "vim": "editors", "nvim": "editors", "nano": "editors", "emacs": "editors", "code": "editors, development",
    "gpg": "security", "openssl": "security, network", "ufw": "network, security", "iptables": "network, security",
    "ffmpeg": "media", "convert": "media", "magick": "media", "man": "documents, shell", "lore": "documents, shell",
    "psql": "databases", "mysql": "databases", "sqlite3": "databases", "redis-cli": "databases",
    "aws": "cloud", "gcloud": "cloud", "az": "cloud", "terraform": "cloud, development",
    "tmux": "shell", "screen": "shell", "echo": "shell, text", "export": "shell", "alias": "shell", "history": "shell",
    "lsusb": "hardware", "lspci": "hardware", "lscpu": "hardware", "dmesg": "system, hardware", "free": "monitoring",
    "uptime": "monitoring, system", "watch": "monitoring, shell", "xclip": "desktop", "xdg-open": "desktop",
}

# destroy data or stop the system (and every `mkfs.*`)
DANGER = {
    "rm", "shred", "wipefs", "mkfs", "mke2fs", "mkswap", "dd", "fdisk", "sfdisk", "cfdisk",
    "gdisk", "sgdisk", "parted", "wipe", "srm", "truncate", "reboot", "shutdown", "poweroff",
    "halt", "kill", "killall", "pkill", "userdel", "deluser", "groupdel", "cryptsetup",
    "lvremove", "vgremove", "pvremove", "mdadm", "format", "diskpart", "del", "erase", "rd",
}
