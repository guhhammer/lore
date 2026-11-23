# llvm-lipo

This command is an alias of `lipo`.

## Usage
- `lipo path/to/binary_file.x86_64 path/to/binary_file.arm64e -create -output path/to/binary_file` : create a universal file from two single-architecture files
- `lipo path/to/binary_file -archs` : list all architectures contained in a universal file
- `lipo path/to/binary_file -detailed_info` : display detailed information about a universal file
- `lipo path/to/binary_file -thin arm64e -output path/to/binary_file.arm64e` : extract a single-architecture file from a universal file

## Categories
files, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/llvm-lipo
