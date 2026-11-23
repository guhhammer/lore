# ocrmypdf

Generate a searchable PDF or PDF/A from a scanned PDF or an image of text.

## Usage
- `ocrmypdf path/to/input path/to/output.pdf` : create a new searchable PDF/A file from a scanned PDF or image file
- `ocrmypdf --skip-text path/to/input.pdf path/to/output.pdf` : skip pages of a mixed-format input PDF file that already contain text
- `ocrmypdf --clean --deskew --rotate-pages path/to/input.pdf path/to/output.pdf` : clean, de-skew, and rotate pages of a poor scan
- `ocrmypdf --tesseract-timeout 0 --optimize 2 --skip-text path/to/input.pdf path/to/output.pdf` : perform lossy optimization on a PDF without performing any OCR
- `ocrmypdf --title "title" --author "author" --subject "subject" --keywords "keyword; key phrase; ..." path/to/input.pdf path/to/output.pdf` : set the metadata of a searchable PDF file
- `ocrmypdf --help` : display help

## Categories
media, documents, files

## Links
- More information: https://ocrmypdf.readthedocs.io/en/latest/cookbook.html
- tldr page: https://tldr.inbrowser.app/pages/common/ocrmypdf
