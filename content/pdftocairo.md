# pdftocairo

Convert PDF files to PNG/JPEG/TIFF/PDF/PS/EPS/SVG using cairo.

## Usage
- `pdftocairo path/to/file.pdf -jpeg` : convert a PDF file to JPEG
- `pdftocairo path/to/file.pdf output.pdf -pdf -expand` : convert to PDF expanding the output to fill the paper
- `pdftocairo path/to/file.pdf output.svg -svg -f first_page -l last_page` : convert to SVG specifying the first/last page to convert
- `pdftocairo path/to/file.pdf output.png -png -r 200` : convert to PNG with 200ppi resolution
- `pdftocairo path/to/file.pdf -tiff -gray -paper A3` : convert to grayscale TIFF setting paper size to A3
- `pdftocairo path/to/file.pdf -png -x x_pixels -y y_pixels` : convert to PNG cropping x and y pixels from the top-left corner

## Categories
documents, media, files

## Links
- More information: https://manned.org/pdftocairo
- tldr page: https://tldr.inbrowser.app/pages/common/pdftocairo
