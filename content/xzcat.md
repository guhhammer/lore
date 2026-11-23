# xzcat

This command is an alias of `xz --decompress --stdout`.

## Usage
- `xz path/to/file` : compress a file into XZ format
- `xz -d path/to/file.xz` : decompress an XZ file
- `xz -F lzma path/to/file` : compress a file using lzma
- `xz -d -F lzma path/to/file.lzma` : decompress an LZMA file
- `xz -d -c path/to/file.xz` : decompress a file and write to `stdout` (implies `--keep`)
- `xz -k path/to/file` : compress a file, but don't delete the original
- `xz -0 path/to/file` : compress a file using the fastest compression
- `xz -9 path/to/file` : compress a file using the best compression

## Categories
archives, files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/xzcat
