# sponge

Soak up the input before writing the output file.

## Usage
- `cat path/to/file | sponge -a path/to/file` : append file content to the source file
- `grep -v '^#' path/to/file | sponge path/to/file` : remove all lines starting with # in a file

## Categories
files

## Links
- More information: https://manned.org/sponge
- tldr page: https://tldr.inbrowser.app/pages/common/sponge
