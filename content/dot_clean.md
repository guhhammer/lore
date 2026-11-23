# dot_clean

Merge `._*` files with corresponding native files.

## Usage
- `dot_clean path/to/directory` : merge all `._*` files recursively
- `dot_clean -f path/to/directory` : don't recursively merge all `._*` in a directory (flat merge)
- `dot_clean -m path/to/directory` : merge and delete all `._*` files
- `dot_clean -n path/to/directory` : only delete `._*` files if there's a matching native file
- `dot_clean -s path/to/directory` : follow symlinks
- `dot_clean -v path/to/directory` : print verbose output

## Categories
files, version-control, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/dot_clean.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/dot_clean
