# shelly

A modern, unified package manager for Arch Linux.

## Usage
- `shelly -Ux` : upgrade all packages from every source (repo, AUR, Flatpak, AppImage)
- `shelly package` : install a package
- `shelly -Rs -c package` : remove an installed package and its orphaned dependencies
- `shelly -Ss -v search_term` : search for a package in the standard repositories
- `shelly -Sa search_term` : search for a package specifically in the AUR
- `shelly -Zs -o` : remove orphaned packages (installed as dependencies but no longer required)

## See also
- pacman
- paru
- yay

## Categories
packages

## Links
- More information: https://www.seafoam-labs.org/shelly-alpm/docs/cli-reference/
- tldr page: https://tldr.inbrowser.app/pages/linux/shelly
