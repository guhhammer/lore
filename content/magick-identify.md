# magick identify

Describe the format and characteristics of image files.

## Usage
- `magick identify path/to/image` : describe the format and basic characteristics of an image
- `magick identify -verbose path/to/image` : describe the format and verbose characteristics of an image
- `magick identify -format "%f,%w,%h\n" *.jpg >path/to/filelist.csv` : collect dimensions of all JPEG files in the current directory and save them into a CSV file

## Categories
containers, media, files

## Links
- More information: https://imagemagick.org/script/identify.php
- tldr page: https://tldr.inbrowser.app/pages/common/magick-identify
- Wikipedia: https://en.wikipedia.org/wiki/ImageMagick
