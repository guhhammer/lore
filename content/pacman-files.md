# pacman-files

Query the local files database.

## Usage
- `sudo pacman -Fy` : update the package database
- `pacman -F filename` : find the package that owns a specific File
- `pacman -Fx 'regex'` : find the package that owns a specific File, using a `regex`
- `pacman -Fq filename` : list only the package names
- `pacman -Fl package` : list the Files owned by a specific package
- `pacman -Fh` : display help

## See also
- pkgfile

## Categories
databases, packages

## Links
- More information: https://manned.org/pacman.8
- tldr page: https://tldr.inbrowser.app/pages/linux/pacman-files
