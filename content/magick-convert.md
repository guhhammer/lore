# magick convert

Convert between image formats, scale, join, and create images, and much more. Note: This tool (previously `convert`) has been replaced by `magick` in ImageMagick 7+.

## Usage
- `magick convert path/to/input_image.jpg path/to/output_image.png` : convert an image from JPEG to PNG
- `magick convert path/to/input_image.png -resize 50% path/to/output_image.png` : scale an image to 50% of its original size
- `magick convert path/to/input_image.png -resize 640x480 path/to/output_image.png` : scale an image keeping the original aspect ratio to a maximum dimension of 640x480
- `magick convert path/to/input_image.png -define jpeg:extent=512kb path/to/output_image.jpg` : scale an image to have a specified file size
- `magick convert -background none path/to/image1.png path/to/image2.png ... -append|+append path/to/output_image.png` : vertically/horizontally append images and have the empty space be transparent
- `magick convert path/to/image1.png path/to/image2.png ... -delay 10 path/to/animation.gif` : create a GIF from a series of images with 100ms delay between them
- `magick convert -size 800x600 "xc:#ff0000" path/to/image.png` : create an image with nothing but a solid red background
- `magick convert path/to/image1.png path/to/image2.png ... path/to/favicon.ico` : create a favicon from several images of different sizes

## Categories
files, containers, media

## Links
- More information: https://imagemagick.org/script/convert.php
- tldr page: https://tldr.inbrowser.app/pages/common/magick-convert
- Wikipedia: https://en.wikipedia.org/wiki/ImageMagick
