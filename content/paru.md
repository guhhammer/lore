# paru

An AUR helper and pacman wrapper.

## Usage
- `paru package_name_or_search_term` : interactively search for and install a package
- `paru` : synchronize and update all packages
- `paru -Sua` : upgrade AUR packages
- `paru -Qqm` : list manually installed packages
- `paru -Rns package` : remove an installed package, its configuration files, and dependencies
- `paru -Si package` : get information about a package
- `paru --getpkgbuild package` : download `PKGBUILD` and other package source files from the AUR or ABS
- `paru -Gp package` : display the `PKGBUILD` file of a package

## See also
- pacman
- yay

## Categories
packages, network

## Links
- More information: https://github.com/Morganamilo/paru#examples
- tldr page: https://tldr.inbrowser.app/pages/linux/paru
