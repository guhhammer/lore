# tabula

Extract tables from PDF files.

## Usage
- `tabula -o file.csv file.pdf` : extract all tables from a PDF to a CSV file
- `tabula -f JSON -o file.json file.pdf` : extract all tables from a PDF to a JSON file
- `tabula -p 1-3,6 file.pdf` : extract tables from pages 1, 2, 3, and 6 of a PDF
- `tabula -g -p 1 file.pdf` : extract tables from page 1 of a PDF, guessing which portion of the page to examine
- `tabula -r file.pdf` : extract all tables from a PDF, using ruling lines to determine cell boundaries
- `tabula -n file.pdf` : extract all tables from a PDF, using blank space to determine cell boundaries

## Categories
documents, media, files

## Links
- More information: https://github.com/tabulapdf/tabula-java#commandline-usage-examples
- tldr page: https://tldr.inbrowser.app/pages/common/tabula
