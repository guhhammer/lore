# pacman-upgrade

Install packages manually from archive files.

## Usage
- `sudo pacman -U path/to/package1.pkg.tar.zst path/to/package2.pkg.tar.zst ...` : install one or more packages from files
- `sudo pacman -U --noconfirm path/to/package.pkg.tar.zst` : install a package without prompting
- `sudo pacman -U --overwrite path/to/file path/to/package.pkg.tar.zst` : overwrite conflicting files during a package installation
- `sudo pacman -Ud path/to/package.pkg.tar.zst` : install a package, skipping the dependency version checks
- `pacman -Up path/to/package.pkg.tar.zst` : fetch and print packages that would be affected by upgrade (does not install any packages)
- `pacman -Uh` : display help

## Categories
packages, archives, files

## Links
- More information: https://manned.org/pacman.8
- tldr page: https://tldr.inbrowser.app/pages/linux/pacman-upgrade
