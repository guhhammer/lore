# apt

Package manager for Debian-based distributions. Intended as a user-friendly alternative to `apt-get` for interactive use. For equivalent commands in other package managers, see <https://wiki.archlinux.org/title/Pacman/Rosetta>.

## Usage
- `sudo apt update` : update the list of available packages and versions (recommended before running other `apt` commands)
- `apt search package` : search packages by name or description
- `apt list package` : search packages by name only (supports wildcards like `*`)
- `apt show package` : show detailed information about a package
- `sudo apt install package` : install a package, or update it to the latest version
- `sudo apt remove package` : remove a package (use `purge` instead to also remove configuration files)
- `sudo apt upgrade` : upgrade all installed packages to their latest versions
- `apt list -i` : list all installed packages

## Categories
packages

## Links
- More information: https://manned.org/apt.8
- tldr page: https://tldr.inbrowser.app/pages/linux/apt
