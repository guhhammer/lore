# aptitude

Debian and Ubuntu package management utility.

## Usage
- `sudo aptitude update` : synchronize list of packages and versions available. This should be run first, before running subsequent `aptitude` commands
- `sudo aptitude install package` : install a new package and its dependencies
- `aptitude search package` : search for a package
- `aptitude search '?installed(package)'` : search for an installed package (`?installed` is an `aptitude` search term)
- `sudo aptitude remove package` : remove a package and all packages depending on it
- `sudo aptitude upgrade` : upgrade installed packages to the newest available versions
- `sudo aptitude full-upgrade` : upgrade installed packages (like `aptitude upgrade`) including removing obsolete packages and installing additional packages to meet new package dependencies
- `sudo aptitude hold '?installed(package)'` : put an installed package on hold to prevent it from being automatically upgraded

## Categories
packages, users

## Links
- More information: https://manned.org/aptitude
- tldr page: https://tldr.inbrowser.app/pages/linux/aptitude
