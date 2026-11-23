# pdftk

PDF toolkit.

## Usage
- `pdftk path/to/input.pdf cat 1-3 5 6-10 output path/to/output.pdf` : extract pages 1-3, 5, and 6-10 from a PDF file and save them as another one
- `pdftk path/to/file1.pdf path/to/file2.pdf cat output path/to/output.pdf` : merge (concatenate) a list of PDF files and save the result as another one
- `pdftk path/to/input.pdf burst output path/to/out_%d.pdf` : split each page of a PDF file into a separate file, with a given filename output pattern
- `pdftk path/to/input.pdf cat 1-endsouth output path/to/output.pdf` : rotate all pages by 180 degrees clockwise
- `pdftk path/to/input.pdf cat 1-2 3east 4-end output path/to/output.pdf` : rotate third page by 90 degrees clockwise and leave others unchanged
- `pdftk A=path/to/fronts.pdf B=path/to/backs.pdf shuffle A1-end Bend-1 output path/to/output.pdf` : interleave two PDFs with one-sided scans of a two-sided document, where the backs were scanned back to front

## Categories
documents, media, files

## Links
- More information: https://www.pdflabs.com/docs/pdftk-man-page/
- tldr page: https://tldr.inbrowser.app/pages/common/pdftk
