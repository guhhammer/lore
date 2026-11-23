# debtap

Convert Debian packages into Arch Linux packages.

## Usage
- `sudo debtap -u` : update debtap database (before the first run)
- `debtap path/to/package.deb` : convert the specified package
- `debtap -q path/to/package.deb` : convert the specified package bypassing all questions, except for editing metadata files
- `debtap -p path/to/package.deb` : generate a PKGBUILD file

## See also
- pacman-upgrade

## Categories
packages, files

## Links
- More information: https://github.com/helixarch/debtap#available-options
- tldr page: https://tldr.inbrowser.app/pages/linux/debtap
