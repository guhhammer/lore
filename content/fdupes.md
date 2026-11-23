# fdupes

Find duplicate files in a set of directories.

## Usage
- `fdupes path/to/directory` : search a single directory
- `fdupes path/to/directory1 path/to/directory2 ...` : search multiple directories
- `fdupes -r path/to/directory` : search a directory recursively
- `fdupes path/to/directory1 -R path/to/directory2` : search multiple directories, one recursively
- `fdupes -rH path/to/directory` : search recursively, considering hardlinks as duplicates
- `fdupes -rd path/to/directory` : search recursively for duplicates and display interactive prompt to pick which ones to keep, deleting the others
- `fdupes -rdN path/to/directory` : search recursively and delete duplicates without prompting

## Categories
files

## Links
- More information: https://github.com/adrianlopezroche/fdupes#introduction
- tldr page: https://tldr.inbrowser.app/pages/common/fdupes
