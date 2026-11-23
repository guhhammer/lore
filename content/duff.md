# duff

Duplicate file finder.

## Usage
- `duff -r path/to/directory` : find duplicate files recursively
- `duff -er path/to/directory` : list all but one file from each cluster of duplicates
- `duff -d sha256 path/to/directory` : use SHA-256 instead of SHA-1
- `duff -t path/to/directory` : compare files byte by byte instead of using digests
- `duff -ar path/to/directory` : include hidden files and directories when searching recursively

## Categories
files

## Links
- More information: https://github.com/elmindreda/duff
- tldr page: https://tldr.inbrowser.app/pages/linux/duff
