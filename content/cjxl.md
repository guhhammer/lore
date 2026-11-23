# cjxl

Convert and compress images to JPEG XL. Accepted input extensions are PNG, APNG, GIF, JPEG, EXR, PPM, PFM, PAM, PGX, and JXL.

## Usage
- `cjxl path/to/image.ext path/to/output.jxl` : convert an image to JPEG XL
- `cjxl path/to/image.jpg path/to/output.jxl` : convert a JPEG to JPEG XL without losing data
- `cjxl -e 1..10 path/to/image.ext path/to/output.jxl` : convert with a specific effort level ('1' - fastest but worst results, '10' - slowest but best results, '7' - default)
- `cjxl -q 0..100 path/to/image.ext path/to/output.jxl` : convert with a specific quality
- `cjxl -q 0..100 -j 0 path/to/image.jpeg path/to/output.jxl` : convert a JPEG to a JPEG XL with specific lossy quality
- `cjxl --faster_decoding 0..4 path/to/image.ext path/to/output.jxl` : adjust decoding speed at the expense of filesize
- `cjxl -h -v -v -v -v` : display an extremely detailed help

## Categories
files, containers, media

## Links
- More information: https://github.com/libjxl/libjxl/blob/main/doc/man/cjxl.txt
- tldr page: https://tldr.inbrowser.app/pages/common/cjxl
