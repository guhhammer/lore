# lipo

Handle Mach-O Universal Binaries.

## Usage
- `lipo path/to/binary_file.x86_64 path/to/binary_file.arm64e -create -output path/to/binary_file` : create a universal file from two single-architecture files
- `lipo path/to/binary_file -archs` : list all architectures contained in a universal file
- `lipo path/to/binary_file -detailed_info` : display detailed information about a universal file
- `lipo path/to/binary_file -thin arm64e -output path/to/binary_file.arm64e` : extract a single-architecture file from a universal file

## Categories
files, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/lipo.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/lipo
