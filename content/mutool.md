# mutool

Convert, query information, and extract data from PDF files.

## Usage
- `mutool convert -o path/to/output%nd.png path/to/input.pdf 1-10` : convert a range of pages to PNGs (Note: `%nd` in the output placeholder must be replaced with a print modifier like `%d` or `%2d`)
- `mutool draw -F txt path/to/input.pdf 2,3,5,...` : convert one or more pages of a PDF into text in `stdout`
- `mutool merge -o path/to/output.pdf path/to/input1.pdf path/to/input2.pdf ...` : concatenate multiple PDF files
- `mutool info path/to/input.pdf` : query information about all content embedded in a PDF
- `mutool extract path/to/input.pdf` : extract all images, fonts, and resources embedded in a PDF to the current directory
- `mutool show path/to/input.pdf outline` : show the outline (table of contents) of a PDF

## Categories
documents, media, files

## Links
- More information: https://mupdf.readthedocs.io/en/latest/tools/mutool.html
- tldr page: https://tldr.inbrowser.app/pages/common/mutool
