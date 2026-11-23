# sips

Apple Scriptable Image Processing System. Raster/Query images and ColorSync ICC Profiles.

## Usage
- `sips --out path/to/output_directory` : specify an output directory so that originals do not get modified
- `sips --resampleHeightWidth 1920 300 image_file.ext` : resample image at specified size, Image aspect ratio may be altered
- `sips --resampleHeightWidthMax 1920 300 image_file.ext` : resample image so height and width aren't greater than specified size (notice the capital Z)
- `sips --resampleWidth 960 path/to/images` : resample all images in a directory to fit a width of 960px (honoring aspect ratio)
- `sips --matchTo "/System/Library/ColorSync/Profiles/Generic RGB Profile.icc" path/to/image.ext --out path/to/output_directory` : convert an image from CMYK to RGB
- `sips --deleteProperty profile --deleteColorManagementProperties path/to/image_file.ext` : remove ColorSync ICC profile from an image

## Categories
containers, media, files, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/sips.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/sips
