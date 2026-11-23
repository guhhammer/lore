# zstdmt

This command is an alias of `zstd --threads 0`.

## Usage
- `zstd path/to/file` : compress a file into a new file with the `.zst` suffix
- `zstd -d path/to/file.zst` : decompress a file
- `zstd -d -c path/to/file.zst` : decompress to `stdout`
- `zstd -1..19 path/to/file` : compress a file specifying the compression level, where 1=fastest, 19=slowest, and 3=default
- `zstd --fast=1..131072 path/to/file` : compress a file using an ultra-fast compression level, where 1=default
- `zstd --ultra -1..22 path/to/file` : unlock higher compression levels (up to 22) using more memory (both for compression and decompression)
- `zstd -T 0` : set the number of working threads to the number of physical CPU cores

## Categories
files, archives, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/zstdmt
