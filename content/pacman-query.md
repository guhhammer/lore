# pacman-query

Query the local package database.

## Usage
- `pacman -Q` : query the local package database and list installed packages and versions
- `pacman -Qeq` : list only packages that were explicitly installed without version numbers
- `pacman -Qo filename` : find which package owns a file
- `pacman -Qi package` : display information about an installed package
- `pacman -Ql package` : display the list of files owned by a specific package
- `pacman -Qdtq` : list orphan packages (installed as dependencies but unrequired (t) by any package and print in quiet mode (only package name is displayed))
- `pacman -Qm` : list installed packages foreign (m) to the repository database
- `pacman -Qu` : list packages that can be upgraded

## Categories
packages, databases

## Links
- More information: https://manned.org/pacman.8
- tldr page: https://tldr.inbrowser.app/pages/linux/pacman-query
