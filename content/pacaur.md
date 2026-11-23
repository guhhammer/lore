# pacaur

A utility for Arch Linux to build and install packages from the Arch User Repository.

## Usage
- `pacaur -Syu` : synchronize and update all packages (includes AUR)
- `pacaur -Syua` : synchronize and update only AUR packages
- `pacaur -S package` : install a new package (includes AUR)
- `pacaur -Rs package` : remove a package and its dependencies (includes AUR packages)
- `pacaur -Ss keyword` : search the package database for a keyword (includes AUR)
- `pacaur -Qs` : list all currently installed packages (includes AUR packages)

## Categories
packages, development, users

## Links
- More information: https://github.com/rmarquis/pacaur#name
- tldr page: https://tldr.inbrowser.app/pages/linux/pacaur
