# inkscape

An SVG (Scalable Vector Graphics) editing program. For Inkscape versions up to 0.92.x, use -e instead of -o.

## Usage
- `inkscape path/to/file.svg` : open an SVG file in the Inkscape GUI
- `inkscape path/to/file.svg -o path/to/file.png` : export an SVG file into a bitmap with the default format (PNG) and the default resolution (96 DPI)
- `inkscape path/to/file.svg -o path/to/file.png -w 600 -h 400` : export an SVG file into a bitmap of 600x400 pixels (aspect ratio distortion may occur)
- `inkscape path/to/file.svg -o path/to/file.png -D` : export the drawing (bounding box of all objects) of an SVG file into a bitmap
- `inkscape path/to/file.svg -i object_id -o object.png` : export a single object, given its ID, into a bitmap
- `inkscape path/to/file.svg -o path/to/file.pdf -T` : export an SVG document to PDF, converting all texts to paths
- `inkscape path/to/file.svg --select=path123 --verb="EditDuplicate;ObjectRotate90;FileSave;FileQuit"` : duplicate the object with id="path123", rotate the duplicate 90 degrees, save the file, and quit Inkscape

## Categories
files, documents

## Links
- More information: https://inkscape.org/doc/inkscape-man.html
- tldr page: https://tldr.inbrowser.app/pages/common/inkscape
- Wikipedia: https://en.wikipedia.org/wiki/Inkscape
