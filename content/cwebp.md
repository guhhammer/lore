# cwebp

Compress an image file to a WebP file.

## Usage
- `cwebp path/to/image_file -o path/to/output.webp` : compress a WebP file with default settings (lossy compression, q = 75) to the output file
- `cwebp path/to/image_file -o path/to/output.webp -q 100` : compress a WebP file with the best lossy compression quality and largest file size
- `cwebp path/to/image_file -o path/to/output.webp -q 0` : compress a WebP file with the worst lossy compression quality and smallest file size
- `cwebp path/to/image_file -o path/to/output.webp -z 9` : compress a WebP file with lossless compression and smallest possible file size
- `cwebp path/to/image_file -o path/to/output.webp -resize width height` : compress a WebP file and apply resize to image (if width or height are 0, scaling preserves aspect ratio)
- `cwebp path/to/image_file -o path/to/output.webp -noalpha` : compress a WebP file and drop alpha channel information

## Categories
files, archives, containers

## Links
- More information: https://developers.google.com/speed/webp/docs/cwebp
- tldr page: https://tldr.inbrowser.app/pages/common/cwebp
