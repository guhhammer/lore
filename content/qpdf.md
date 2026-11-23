# qpdf

Versatile PDF transformation software.

## Usage
- `qpdf --empty --pages path/to/input.pdf 1-3,5,6-10 -- path/to/output.pdf` : extract pages 1-3, 5, and 6-10 from a PDF file and save them as another one
- `qpdf --empty --pages path/to/file1.pdf file2.pdf ... -- path/to/output.pdf` : merge (concatenate) all the pages of multiple PDF files and save the result as a new PDF
- `qpdf --empty --pages path/to/file1.pdf 1,6-8 path/to/file2.pdf 3,4,5 -- path/to/output.pdf` : merge (concatenate) given pages from a list of PDF files and save the result as a new PDF
- `qpdf --split-pages=n path/to/input.pdf path/to/out_%d.pdf` : write each group of `n` pages to a separate output file with a given filename pattern
- `qpdf --rotate=90:2,4,6 --rotate=180:7-8 path/to/input.pdf path/to/output.pdf` : rotate certain pages of a PDF with a given angle
- `qpdf --password=password --decrypt path/to/input.pdf path/to/output.pdf` : remove the password from a password-protected file

## Categories
documents, media, files

## Links
- More information: https://manned.org/qpdf
- tldr page: https://tldr.inbrowser.app/pages/common/qpdf
