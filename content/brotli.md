# brotli

Compress/uncompress files with Brotli compression.

## Usage
- `brotli path/to/file` : compress a file, creating a compressed version next to the file
- `brotli -d path/to/file.br` : decompress a file, creating an uncompressed version next to the file
- `brotli path/to/file -o path/to/compressed_output_file.br` : compress a file specifying the output filename
- `brotli -d path/to/compressed_file.br -o path/to/output_file` : decompress a Brotli file specifying the output filename
- `brotli -q 11 path/to/file -o path/to/compressed_output_file.br` : specify the compression quality (1=fastest (worst), 11=slowest (best))

## Categories
files, archives

## Links
- More information: https://manned.org/brotli
- tldr page: https://tldr.inbrowser.app/pages/common/brotli
