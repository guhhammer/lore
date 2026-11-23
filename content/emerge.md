# emerge

Gentoo Linux package manager utility. For equivalent commands in other package managers, see <https://wiki.archlinux.org/title/Pacman/Rosetta>.

## Usage
- `sudo emerge --sync` : synchronize all packages
- `sudo emerge -avuDN @world` : update all packages, including dependencies
- `sudo emerge --resume --skipfirst` : resume a failed update, skipping the failing package
- `sudo emerge -av package` : install a new package, with confirmation
- `sudo emerge -avc package` : remove a package and its dependencies with confirmation
- `sudo emerge -avc` : remove orphaned packages (installed as dependencies but no longer required by any package)
- `emerge -S keyword` : search the package database for a keyword

## Categories
packages, users, network

## Links
- More information: https://wiki.gentoo.org/wiki/Portage#emerge
- tldr page: https://tldr.inbrowser.app/pages/linux/emerge
