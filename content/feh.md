# feh

Lightweight image viewing utility.

## Usage
- `feh path/to/images` : view images locally or using a URL
- `feh -r path/to/images` : view images recursively
- `feh -d path/to/images` : view images and display the file name at the top-left of the images
- `feh -x path/to/images` : view images without window borders
- `feh --on-last-slide hold|quit|resume path/to/images` : set the behavior when reaching the beginning or end of the image list
- `feh -D seconds path/to/images` : use a specific slideshow cycle delay
- `feh --bg-center|fill|max|scale|tile path/to/image` : use a specific wallpaper mode (centered, filled, maximized, scaled, or tiled)
- `feh -m -E 150 -y 150 --index-info "%nn%wx%h" -o path/to/montage_image.png` : create a montage of all images within a directory, outputting as a new image

## Categories
containers, media, files

## Links
- More information: https://man.finalrewind.org/1/feh/
- tldr page: https://tldr.inbrowser.app/pages/common/feh
