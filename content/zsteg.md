# zsteg

Steganography detection tool for PNG and BMP file formats. It detects LSB steganography, ZLIB-compressed data, OpenStego, Camouflage, and LSB with the Eratosthenes set.

## Usage
- `zsteg path/to/image.png` : detect embedded data in a PNG
- `zsteg --all path/to/image.bmp` : detect embedded data in a BMP image, using all known methods
- `zsteg --msb --order yx path/to/image.png` : detect embedded data in a PNG, iterating pixels vertically and using MSB first
- `zsteg --bits 1,2,3|1-3 path/to/image.bmp` : detect embedded data in a BMP image, specifying the bits to consider
- `zsteg --prime --invert path/to/image.png` : detect embedded data in a PNG, extracting only prime pixels and inverting bits
- `zsteg --min-str-len 10 --strings first|all|longest|none path/to/image.bmp` : detect embedded data in a BMP image, specifying the minimum length of the strings to be found and the find mode

## Categories
containers, files, media

## Links
- More information: https://github.com/zed-0xff/zsteg
- tldr page: https://tldr.inbrowser.app/pages/common/zsteg
