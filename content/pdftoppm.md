# pdftoppm

Convert PDF document pages to portable Pixmap (image formats).

## Usage
- `pdftoppm -f n -l m path/to/file.pdf image_name_prefix` : specify the range of pages to convert (`n` - first page, `m` - last page)
- `pdftoppm -singlefile path/to/file.pdf image_name_prefix` : convert only the first page of a PDF
- `pdftoppm -mono path/to/file.pdf image_name_prefix` : generate a monochrome PBM file (instead of a color PPM file)
- `pdftoppm -gray path/to/file.pdf image_name_prefix` : generate a grayscale PGM file (instead of a color PPM file)
- `pdftoppm -png path/to/file.pdf image_name_prefix` : generate a PNG file instead a PPM file

## Categories
files, media, documents

## Links
- More information: https://manned.org/pdftoppm
- tldr page: https://tldr.inbrowser.app/pages/linux/pdftoppm
