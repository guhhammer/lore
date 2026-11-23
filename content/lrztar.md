# lrztar

A wrapper for `lrzip` to simplify compression of directories.

## Usage
- `lrztar path/to/directory` : archive a directory with `tar`, then compress
- `lrztar -z path/to/directory` : archive a directory with `tar`, then compress with ZPAQ (extreme compression, but very slow)
- `lrztar -o path/to/file path/to/directory` : specify the output file
- `lrztar -p 8 path/to/directory` : override the number of processor threads to use
- `lrztar -f path/to/directory` : force overwriting of existing files

## See also
- tar
- lrzuntar
- lrunzip

## Categories
files, archives

## Links
- More information: https://manned.org/lrztar
- tldr page: https://tldr.inbrowser.app/pages/linux/lrztar
