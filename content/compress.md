# compress

Compress files using the Unix `compress` command.

## Usage
- `compress path/to/file1 path/to/file2 ...` : compress specific files
- `compress -f path/to/file1 path/to/file2 ...` : compress specific files, ignore non-existent ones
- `compress -b 9..16` : specify the maximum compression bits (9-16 bits)
- `compress -c path/to/file` : write to `stdout` (no files are changed)
- `compress -d path/to/file` : decompress files (functions like `uncompress`)
- `compress -v path/to/file` : display compression percentage

## Categories
archives, files

## Links
- More information: https://manned.org/compress
- tldr page: https://tldr.inbrowser.app/pages/linux/compress
