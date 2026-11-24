# magick

Create, edit, compose, or convert between image formats. This tool replaces `convert` in ImageMagick 7+. See `magick convert` to use the old tool in versions 7+. Some subcommands such as `mogrify` have their own usage documentation.

## Usage
- `magick path/to/input_image.png path/to/output_image.jpg` : convert between image formats
- `magick path/to/input_image.jpg -resize 100x100 path/to/output_image.jpg` : resize an image, making a new copy
- `magick path/to/input_image.png -resize 50% path/to/output_image.png` : resize an image by a percentage
- `magick path/to/input_image.png -define jpeg:extent=512kb path/to/output_image.jpg` : scale an image to have a specified file size
- `magick convert -background none path/to/image1.png path/to/image2.png ... -append|+append path/to/output_image.png` : vertically/horizontally append images and have the empty space be transparent
- `magick *.jpg path/to/images.gif` : create a GIF out of all JPEG images in the current directory
- `magick -size 640x480 pattern:checkerboard path/to/checkerboard.png` : create a checkerboard pattern
- `magick *.jpg -adjoin path/to/file.pdf` : create a PDF file out of all JPEG images in the current directory

## Categories
media

## Links
- More information: https://imagemagick.org/script/magick.php
- tldr page: https://tldr.inbrowser.app/pages/common/magick
- Wikipedia: https://en.wikipedia.org/wiki/ImageMagick
