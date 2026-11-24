# exif

Show and change EXIF information in JPEG files.

## Usage
- `exif path/to/image.jpg` : show all recognized EXIF information in an image
- `exif -l --no-fixup path/to/image.jpg` : show a table listing known EXIF tags and whether each one exists in an image
- `exif -e -o path/to/thumbnail.jpg path/to/image.jpg` : extract the image thumbnail into a separate file
- `exif --ifd 0 -t "Model" -m path/to/image.jpg` : show the raw contents of the "Model" tag in the given image
- `exif -o path/to/new.jpg --ifd 0 -t "Artist" --set-value "John Smith" --no-fixup path/to/image.jpg` : change the value of the "Artist" tag to John Smith and save to `new.jpg`

## Categories
containers, media, files

## Links
- More information: https://manned.org/exif
- tldr page: https://tldr.inbrowser.app/pages/linux/exif
- Wikipedia: https://en.wikipedia.org/wiki/Exif
