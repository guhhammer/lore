# paccache

A `pacman` cache cleaning utility.

## Usage
- `paccache -r` : remove all but the 3 most recent package versions from the `pacman` cache
- `paccache -rk num_versions` : set the number of package versions to keep
- `paccache -d` : simulate the cleaning and show the number of candidate packages for deletion
- `paccache -m path/to/directory` : move candidate packages to a directory instead of deleting them

## Categories
files, packages

## Links
- More information: https://manned.org/paccache
- tldr page: https://tldr.inbrowser.app/pages/linux/paccache
