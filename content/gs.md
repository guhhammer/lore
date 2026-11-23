# gs

GhostScript, a PDF and PostScript interpreter.

## Usage
- `gs -dQUIET -dBATCH file.pdf` : view a file
- `gs -dNOPAUSE -dQUIET -dBATCH -sDEVICE=pdfwrite -dPDFSETTINGS=/ebook -sOutputFile=output.pdf input.pdf` : reduce PDF file size to 150 dpi images for reading on an e-book device
- `gs -dQUIET -dBATCH -dNOPAUSE -sDEVICE=jpeg -r150 -dFirstPage=1 -dLastPage=3 -sOutputFile=output_%d.jpg input.pdf` : convert PDF file (pages 1 through 3) to an image with 150 dpi resolution
- `gs -dQUIET -dBATCH -dNOPAUSE -sDEVICE=pdfwrite -sOutputFile=output.pdf input.pdf` : extract pages from a PDF file
- `gs -dQUIET -dBATCH -dNOPAUSE -sDEVICE=pdfwrite -sOutputFile=output.pdf input1.pdf input2.pdf` : merge PDF files
- `gs -dQUIET -dBATCH -dNOPAUSE -sDEVICE=pdfwrite -sOutputFile=output.pdf input.ps` : convert from PostScript file to PDF file

## Categories
media, documents, files

## Links
- More information: https://manned.org/gs
- tldr page: https://tldr.inbrowser.app/pages/linux/gs
