# systemd-sysusers

Create system users and groups. If the config file is not specified, files in the `sysusers.d` directories are used.

## Usage
- `systemd-sysusers path/to/file` : create users and groups from a specific configuration file
- `systemd-sysusers --dry-run path/to/file` : simulate processing configuration files, printing what would be done
- `systemd-sysusers --cat-config` : print the contents of all configuration files (before each file, its name is printed as a comment)
- `systemd-sysusers` : create users based on files listed in the previous command

## Categories
files, users, system

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-sysusers
