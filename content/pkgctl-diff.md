# pkgctl diff

Compare package files using different modes.

## Usage
- `pkgctl diff -l path/to/file|pkgname` : compare package files in tar content list different mode (default)
- `pkgctl diff -d path/to/file|pkgname` : compare package files in diffoscope different mode
- `pkgctl diff -p path/to/file|pkgname` : compare package files in `.PKGINFO` different mode
- `pkgctl diff -b path/to/file|pkgname` : compare package files in `.BUILDINFO` different mode

## Categories
version-control, files, packages

## Links
- More information: https://manned.org/pkgctl-diff
- tldr page: https://tldr.inbrowser.app/pages/linux/pkgctl-diff
