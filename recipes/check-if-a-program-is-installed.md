# check if a program is installed

Find whether a command exists, where it is, and which package it came from.

## Steps
- `command -v git` : its path, or nothing if it is not there (use this in scripts)
- `type -a python3` : every python3 the shell can find, aliases and functions too
- `git --version` : its version
- `dpkg -l | grep NAME` : installed by apt? (Debian, Ubuntu)
- `apt policy NAME` : installed and available versions

## Notes
- lore shows every command it finds, and each page's VERSION says the package.

## See also
- command
- type
- dpkg
- apt

## Categories
shell packages
