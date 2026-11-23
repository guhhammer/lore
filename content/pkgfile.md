# pkgfile

Search files from packages in the official repositories on Arch-based systems.

## Usage
- `sudo pkgfile --update` : synchronize the pkgfile database
- `pkgfile filename` : search for a package that owns a specific file
- `pkgfile --list package` : list all files provided by a package
- `pkgfile --list --binaries package` : list executables provided by a package
- `pkgfile --ignorecase filename` : search for a package that owns a specific file using case-insensitive matching
- `pkgfile --binaries filename` : search for a package that owns a specific file in the `bin` or `sbin` directory
- `pkgfile --verbose filename` : search for a package that owns a specific file, displaying the package version
- `pkgfile --repo repository_name filename` : search for a package that owns a specific file in a specific repository

## See also
- pacman files

## Categories
packages, files

## Links
- More information: https://manned.org/pkgfile
- tldr page: https://tldr.inbrowser.app/pages/linux/pkgfile
