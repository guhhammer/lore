# pdfgrep

Search text in PDF files.

## Usage
- `pdfgrep pattern file.pdf` : find lines that match pattern in a PDF
- `pdfgrep -H -n pattern file.pdf` : include file name and page number for each matched line
- `pdfgrep -m 3 -i '^file_name' file.pdf` : do a case-insensitive search for lines that begin with `file_name` and return the first 3 matches
- `pdfgrep -r pattern` : find pattern in files with a `.pdf` extension in the current directory recursively
- `pdfgrep -r --include '*book.pdf' pattern` : find pattern on files that match a specific `glob` in the current directory recursively

## Categories
text, documents, media

## Links
- More information: https://pdfgrep.org/doc.html
- tldr page: https://tldr.inbrowser.app/pages/common/pdfgrep
