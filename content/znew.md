# znew

Recompress files from `.Z` to `gzip` format.

## Usage
- `znew path/to/file1.Z` : recompress a file from `.Z` to `gzip` format
- `znew -v path/to/file1.Z path/to/file2.Z ...` : recompress multiple files and display the achieved size reduction % per file
- `znew -9 path/to/file1.Z` : recompress a file using the slowest compression method (for optimal compression)
- `znew -K path/to/file1.Z` : recompress a file, Keeping the `.Z` file if it is smaller than the `gzip` file

## Categories
files, archives

## Links
- More information: https://manned.org/znew
- tldr page: https://tldr.inbrowser.app/pages/common/znew
