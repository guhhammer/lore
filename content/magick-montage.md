# magick montage

Tile images into a customizable grid.

## Usage
- `magick montage path/to/image1.jpg path/to/image2.jpg ... path/to/montage.jpg` : tile images into a grid, automatically resizing images larger than the grid cell size
- `magick montage path/to/image1.jpg path/to/image2.jpg ... -geometry +0+0 path/to/montage.jpg` : tile images into a grid, automatically calculating the grid cell size from the largest image
- `magick montage path/to/image1.jpg path/to/image2.jpg ... -geometry 640x480+0+0 path/to/montage.jpg` : specify the grid cell size and resize images to fit it before tiling
- `magick montage path/to/image1.jpg path/to/image2.jpg ... -geometry +0+0 -tile 2x3 montage_%d.jpg` : limit the number of rows and columns in the grid, causing input images to overflow into multiple output montages
- `magick montage path/to/image1.jpg path/to/image2.jpg ... -geometry +0+0 -resize 640x480^ -gravity center -crop 640x480+0+0 path/to/montage.jpg` : resize and crop images to fill their grid cells before tiling

## Categories
files, containers, media

## Links
- More information: https://imagemagick.org/script/montage.php
- tldr page: https://tldr.inbrowser.app/pages/common/magick-montage
- Wikipedia: https://en.wikipedia.org/wiki/ImageMagick
