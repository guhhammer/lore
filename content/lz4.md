# lz4

Compress or decompress `.lz4` files.

## Usage
- `lz4 path/to/file` : compress a file
- `lz4 -d file.lz4` : decompress a file
- `lz4 -dc file.lz4` : decompress a file and write to `stdout`
- `tar cvf - path/to/directory | lz4 - dir.tar.lz4` : package and compress a directory and its contents
- `lz4 -dc dir.tar.lz4 | tar -xv` : decompress and unpack a directory and its contents
- `lz4 -12 path/to/file` : compress a file using the best compression

## Categories
archives, files

## Links
- More information: https://github.com/lz4/lz4
- tldr page: https://tldr.inbrowser.app/pages/common/lz4
