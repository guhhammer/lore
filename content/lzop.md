# lzop

Compress or decompress files with LZO compression.

## Usage
- `lzop path/to/file` : compress a file into a new file with the `.lzo` suffix
- `lzop -d path/to/file.lzo` : decompress a file
- `lzop -0..9 path/to/file` : compress a file, while specifying the compression level. 0 = Worst, 9 = Best (Default level is 3)
- `lzop -9 path/to/file` : compress a file with the best compression level
- `lzop -1 path/to/file` : compress a file with the fastest compression level

## Categories
files, archives

## Links
- More information: https://www.lzop.org/lzop_man.php
- tldr page: https://tldr.inbrowser.app/pages/common/lzop
- Wikipedia: https://en.wikipedia.org/wiki/Lempel%E2%80%93Ziv%E2%80%93Oberhumer
