# p7zip

Wrapper of 7-Zip file archiver with high compression ratio. Internally executes either 7za or 7zr command.

## Usage
- `p7zip path/to/file` : archive a file, replacing it with a 7zipped compressed version
- `p7zip -k path/to/file` : archive a file keeping the input file
- `p7zip -d compressed.ext.7z` : decompress a file, replacing it with the original uncompressed version
- `p7zip -d -k compressed.ext.7z` : decompress a file keeping the input file
- `p7zip -f path/to/file` : skip some checks and force compression or decompression

## Categories
files, archives

## Links
- More information: https://manned.org/p7zip
- tldr page: https://tldr.inbrowser.app/pages/common/p7zip
