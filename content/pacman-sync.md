# pacman-sync

Synchronize packages from remote repositories.

## Usage
- `sudo pacman -S package` : install a new package
- `sudo pacman -Syu` : synchronize and refresh (y) the package database along with a sysupgrade (add `--downloadonly` to only download the packages and not update them)
- `sudo pacman -Syu --noconfirm package` : update and upgrade all packages and install a new one without prompting
- `pacman -Ss "search_pattern"` : search the package database for a `regex` or keyword
- `pacman -Si package` : display information about a package
- `sudo pacman -Syu --overwrite path/to/file` : overwrite conflicting files during a package update
- `sudo pacman -Sc` : remove not installed packages and unused repositories from the cache (use the flags `Scc` to clean all packages)
- `sudo pacman -S package=version` : specify the package version that should be installed

## Categories
packages, users, network

## Links
- More information: https://manned.org/pacman.8
- tldr page: https://tldr.inbrowser.app/pages/linux/pacman-sync
