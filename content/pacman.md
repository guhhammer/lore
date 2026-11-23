# pacman

Arch Linux package manager utility. For equivalent commands in other package managers, see <https://wiki.archlinux.org/title/Pacman/Rosetta>.

## Usage
- `sudo pacman -Syu` : synchronize and update all packages
- `sudo pacman -S package` : install a new package
- `sudo pacman -Rs package` : remove a package and its dependencies
- `pacman -Ss "search_pattern"` : search (s) the package database for a `regex` or keyword
- `pacman -F "file_name"` : search the database for packages containing a specific File
- `pacman -Qe` : list only the explicitly installed packages and versions
- `pacman -Qtdq` : list orphan packages (installed as dependencies but not actually required by any package)
- `sudo pacman -Scc` : empty the entire `pacman` cache

## See also
- pacman-sync
- pacman-remove
- pacman-query
- pacman-upgrade
- pacman-files
- pacman-database
- pacman-deptest
- pacman-key
- pacman-mirrors

## Categories
packages

## Links
- More information: https://manned.org/pacman.8
- tldr page: https://tldr.inbrowser.app/pages/linux/pacman
