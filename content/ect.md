# ect

Efficient Compression Tool. File optimizer written in C++. It supports PNG, JPEG, gzip, and Zip files.

## Usage
- `ect path/to/file.png` : compress a file
- `ect -9 --mt-deflate path/to/file.zip` : compress a file with specified compression level and multithreading (1=Fastest (Worst), 9=Slowest (Best), default is 3)
- `ect -recurse path/to/directory` : compress all files in a directory recursively
- `ect -keep path/to/file.png` : compress a file, keeping the original modification time
- `ect -strip path/to/file.png` : compress a file, stripping metadata

## Categories
files, archives

## Links
- More information: https://github.com/fhanau/Efficient-Compression-Tool
- tldr page: https://tldr.inbrowser.app/pages/common/ect
