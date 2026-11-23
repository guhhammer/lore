# uncompress

Uncompress files compressed using the Unix `compress` command.

## Usage
- `uncompress path/to/file1.Z path/to/file2.Z ...` : uncompress specific files
- `uncompress -f path/to/file1.Z path/to/file2.Z ...` : uncompress specific files while ignoring non-existent ones
- `uncompress -c path/to/file1.Z path/to/file2.Z ...` : write to `stdout` (no files are changed and no `.Z` files are created)
- `uncompress -v path/to/file1.Z path/to/file2.Z ...` : uncompress files verbosely (write to `stderr` about percentage reduction or expansion)

## Categories
files, archives

## Links
- More information: https://manned.org/uncompress
- tldr page: https://tldr.inbrowser.app/pages/linux/uncompress
