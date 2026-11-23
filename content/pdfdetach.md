# pdfdetach

List or extract attachments (embedded files) from a PDF file.

## Usage
- `pdfdetach list -enc UTF-8 path/to/input.pdf` : list all attachments in a file with a specific text encoding
- `pdfdetach -save number path/to/input.pdf` : save specific embedded file by specifying its number
- `pdfdetach -savefile name path/to/input.pdf` : save specific embedded file by specifying its name
- `pdfdetach -save number -o path/to/output path/to/input.pdf` : save the embedded file with a custom output filename
- `pdfdetach -save number -opw|-upw password path/to/input.pdf` : save the attachment from a file secured by owner/user password

## See also
- pdfattach
- pdfimages
- pdfinfo

## Categories
files, documents, media

## Links
- More information: https://manned.org/pdfdetach
- tldr page: https://tldr.inbrowser.app/pages/linux/pdfdetach
