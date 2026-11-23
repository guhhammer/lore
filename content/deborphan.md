# deborphan

Display orphan packages on operating systems using the APT package manager.

## Usage
- `deborphan` : display library packages (from the "libs" section of the package repository) which are not required by another package
- `deborphan --guess-all` : list orphan packages from the "libs" section as well as orphan packages that have a name that looks like a library name
- `deborphan -n` : find packages which are only recommended or suggested (but not required) by another package

## Categories
packages, desktop

## Links
- More information: https://manned.org/deborphan
- tldr page: https://tldr.inbrowser.app/pages/linux/deborphan
