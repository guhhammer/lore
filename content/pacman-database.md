# pacman-database

Operate on the Arch Linux package database. Modify certain attributes of the installed packages.

## Usage
- `sudo pacman -D --asdeps package` : mark a package as implicitly installed
- `sudo pacman -D --asexplicit package` : mark a package as explicitly installed
- `pacman -Dk` : check that all the package dependencies are installed
- `pacman -Dkk` : check the sync Database to ensure all specified dependencies of downloadable packages are available
- `pacman -Dkq` : check and display in quiet mode (only error messages are displayed)
- `pacman -Dh` : display help

## Categories
packages, databases

## Links
- More information: https://manned.org/pacman.8
- tldr page: https://tldr.inbrowser.app/pages/linux/pacman-database
