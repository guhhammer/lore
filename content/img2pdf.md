# img2pdf

Losslessly convert raster images to a PDF file. Some supported image formats are: GIF, JPEG, JPEG2000, PNG, GIF, and TIFF.

## Usage
- `img2pdf path/to/image1.ext path/to/image2.ext ... --output path/to/file.pdf` : convert one or more images to a single PDF, each image being on its own page
- `img2pdf path/to/file.gif --first-frame-only --output path/to/file.pdf` : convert only the first frame of a multi-frame image to PDF
- `img2pdf path/to/image.ext --auto-orient --pagesize A4^T --border 2cm:5.1cm --output path/to/file.pdf` : auto orient the image, use a specific page size in landscape mode, and set a border of specific sizes horizontally and vertically
- `img2pdf path/to/image.ext --pagesize 30cmx20cm --imgsize 10cmx15cm --fit shrink --output path/to/file.pdf` : shrink only larger images to a rectangle of specified dimensions inside a page with a specific size
- `img2pdf path/to/image.ext --title title --author author --creationdate 1970-01-31 --keywords keyword1 keyword2 --subject subject --output path/to/file.pdf` : convert an image to PDF and specify metadata for the resulting file

## Categories
media, files, containers

## Links
- More information: https://gitlab.mister-muffin.de/josch/img2pdf
- tldr page: https://tldr.inbrowser.app/pages/common/img2pdf
