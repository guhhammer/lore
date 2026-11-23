# magick mogrify

Perform operations on multiple images, such as resizing, cropping, flipping, and adding effects. Changes are applied directly to the original file.

## Usage
- `magick mogrify -resize 50% *.jpg` : resize all JPEG images in the directory to 50% of their initial size
- `magick mogrify -resize 800x600 DSC*` : resize all images starting with `DSC` to 800x600
- `magick mogrify -format jpg *.png` : convert all PNGs in the directory to JPEG
- `magick mogrify -modulate 100,50 *` : halve the saturation of all image files in the current directory
- `magick mogrify -modulate 200 *` : double the brightness of all image files in the current directory
- `magick mogrify -layers 'optimize' -fuzz 7% *.gif` : reduce file sizes of all GIF images in the current directory by reducing quality
- `magick mogrify -help` : display help

## Categories
files, containers, media

## Links
- More information: https://imagemagick.org/script/mogrify.php
- tldr page: https://tldr.inbrowser.app/pages/common/magick-mogrify
