# hardlink

Consolidate duplicate files by replacing them with hard links. Part of `util-linux`, useful for saving disk space when identical files exist across directories.

## Usage
- `hardlink -n path/to/directory` : simulate hardlinking duplicate files without modifying them
- `hardlink path/to/directory` : hardlink duplicate files in a directory
- `hardlink path/to/directory1 path/to/directory2 ...` : hardlink duplicate files in multiple directories
- `hardlink -v path/to/directory` : display verbose output while hardlinking
- `hardlink -s size path/to/directory` : only consider files with a minimum size

## Categories
files, disks

## Links
- More information: https://manned.org/hardlink
- tldr page: https://tldr.inbrowser.app/pages/linux/hardlink
