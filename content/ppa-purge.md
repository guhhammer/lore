# ppa-purge

Disable a PPA and revert its packages to the official versions.

## Usage
- `sudo ppa-purge ppa:owner/ppa_name` : disable a PPA and downgrade its packages to the official versions
- `sudo ppa-purge -o owner` : disable a PPA given only its owner (the PPA name defaults to `ppa`)
- `sudo ppa-purge -o owner -p ppa_name` : disable a PPA given its owner and pPA name
- `sudo ppa-purge -y ppa:owner/ppa_name` : answer yes to all package manager prompts
- `sudo ppa-purge -d distribution ppa:owner/ppa_name` : use a specific distribution instead of the detected one
- `ppa-purge -h` : display help

## Categories
packages, users

## Links
- More information: https://manned.org/ppa-purge
- tldr page: https://tldr.inbrowser.app/pages/linux/ppa-purge
