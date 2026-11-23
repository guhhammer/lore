# apt-get

Debian and Ubuntu package management utility. Search for packages using `apt-cache`. It is recommended to use `apt` when used interactively in Ubuntu versions 16.04 and later.

## Usage
- `sudo apt-get update` : update the list of available packages and versions (it's recommended to run this before other `apt-get` commands)
- `sudo apt-get install package` : install a package, or update it to the latest available version
- `sudo apt-get remove package` : remove a package
- `sudo apt-get purge package` : remove a package and its configuration files
- `sudo apt-get upgrade` : upgrade all installed packages to their newest available versions
- `sudo apt-get autoclean` : clean the local repository - removing package files (`.deb`) from interrupted downloads that can no longer be downloaded
- `sudo apt-get autoremove` : remove all packages that are no longer needed
- `sudo apt-get dist-upgrade` : upgrade installed packages (like `upgrade`), but remove obsolete packages and install additional packages to meet new dependencies

## Categories
packages

## Links
- More information: https://manned.org/apt-get.8
- tldr page: https://tldr.inbrowser.app/pages/linux/apt-get
