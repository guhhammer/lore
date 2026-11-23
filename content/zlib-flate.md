# zlib-flate

Raw zlib compression and decompression program. Part of `qpdf`.

## Usage
- `zlib-flate <path/to/input_file -compress >path/to/compressed.zlib` : compress a file
- `zlib-flate <path/to/compressed.zlib -uncompress >path/to/output_file` : uncompress a file
- `zlib-flate <path/to/input_file -compress=0..9 >path/to/compressed.zlib` : compress a file with a specified compression level. 0=Fastest (Worst), 9=Slowest (Best)

## Categories
files, archives

## Links
- More information: https://manned.org/zlib-flate
- tldr page: https://tldr.inbrowser.app/pages/common/zlib-flate
