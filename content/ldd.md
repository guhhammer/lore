# ldd

Display shared library dependencies of a binary. Do not use on an untrusted binary, use objdump for that instead.

## Usage
- `ldd path/to/binary` : display shared library dependencies of a binary
- `ldd -v path/to/binary` : display all information about dependencies
- `ldd -u path/to/binary` : display unused direct dependencies
- `ldd -d path/to/binary` : report missing data objects and perform data relocations
- `ldd -r path/to/binary` : report missing data objects and functions, and perform relocations for both

## Categories
desktop, packages, files

## Links
- More information: https://manned.org/ldd
- tldr page: https://tldr.inbrowser.app/pages/linux/ldd
- Wikipedia: https://en.wikipedia.org/wiki/Ldd_(Unix)
