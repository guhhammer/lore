# pkgutil

Query and manipulate Mac OS X Installer packages and receipts.

## Usage
- `pkgutil --pkgs` : list package IDs for all installed packages
- `pkgutil --check-signature path/to/file.pkg` : verify cryptographic signatures of a package file
- `pkgutil --files com.microsoft.Word` : list all the files for an installed package given its ID
- `pkgutil --expand-full path/to/file.pkg path/to/directory` : extract the contents of a package file into a directory

## Categories
files, packages, databases, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/pkgutil.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/pkgutil
