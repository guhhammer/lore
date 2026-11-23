# dwebp

`dwebp` decompresses WebP files into PNG, PAM, PPM, or PGM images. Animated WebP files are not supported.

## Usage
- `dwebp path/to/input.webp -o path/to/output.png` : convert a WebP file into a PNG file
- `dwebp path/to/input.webp -bmp|-tiff|-pam|-ppm|-pgm|-yuv -o path/to/output` : convert a WebP file into a specific filetype
- `dwebp path/to/input.webp -o path/to/output.png -mt` : convert a WebP file, using multi-threading if possible
- `dwebp input.webp -o output.png -crop x_pos y_pos width height -scale width height` : convert a WebP file, but also crop and scale at the same time
- `dwebp path/to/input.webp -o path/to/output.png -flip` : convert a WebP file and flip the output
- `dwebp path/to/input.webp -o path/to/output.png -nofilter` : convert a WebP file and don't use in-loop filtering to speed up the decoding process

## Categories
files, containers, media

## Links
- More information: https://developers.google.com/speed/webp/docs/dwebp/
- tldr page: https://tldr.inbrowser.app/pages/common/dwebp
