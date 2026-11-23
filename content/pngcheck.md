# pngcheck

Forensics tool for validating the integrity of PNG based (PNG, JNG, MNG) image files. Can also extract embedded images and text from a file.

## Usage
- `pngcheck path/to/image.png` : verify the integrity of an image file (width, height, and color depth)
- `pngcheck -c path/to/image.png` : print information for an image with colorized output
- `pngcheck -cvt path/to/image.png` : print verbose information for an image
- `cat path/to/image.png | pngcheck -cvt` : receive an image from `stdin` and display detailed information
- `pngcheck -s path/to/image.png` : search for PNGs within a specific file and display information about them
- `pngcheck -x path/to/image.png` : search for PNGs within another file and extract them

## Categories
media, containers, files

## Links
- More information: https://manned.org/pngcheck
- tldr page: https://tldr.inbrowser.app/pages/common/pngcheck
