# aurman

An Arch Linux utility to build and install packages from the Arch User Repository.

## Usage
- `aurman -S -y -u` : synchronize and update all packages
- `aurman -S -y -u --noedit` : synchronize and update all packages without show changes of `PKGBUILD` files
- `aurman -S package` : install a new package
- `aurman -S --noedit package` : install a new package without show changes of `PKGBUILD` files
- `aurman -S --noedit --noconfirm package` : install a new package without prompting
- `aurman -S -s keyword` : search the package database for a keyword from the official repositories and AUR
- `aurman --remove --recursive --nosave package` : remove a package and its dependencies
- `aurman -S -c` : clear the package cache (use two `--clean` flags to clean all packages)

## See also
- pacman

## Categories
packages, development, users

## Links
- More information: https://github.com/polygamma/aurman#syntax
- tldr page: https://tldr.inbrowser.app/pages/linux/aurman
