# repo-add

Package database maintenance utility which enables installation of said package via Pacman.

## Usage
- `repo-add path/to/database.db.tar.gz` : create an empty repository
- `repo-add path/to/database.db.tar.gz package1.pkg.tar.zst package2.pkg.tar.zst ...` : add packages to the repository
- `repo-add -R path/to/database.db.tar.gz *.pkg.tar.zst` : add all package binaries in the current directory and remove any outdated package files
- `repo-add -q path/to/database.db.tar.gz *.pkg.tar.zst` : add all package binaries in the current directory in silent mode except for warning and error messages
- `repo-add --nocolor path/to/database.db.tar.gz *.pkg.tar.zst` : add all package binaries in the current directory without showing color

## See also
- repo-remove

## Categories
packages, archives, databases

## Links
- More information: https://manned.org/repo-add
- tldr page: https://tldr.inbrowser.app/pages/linux/repo-add
