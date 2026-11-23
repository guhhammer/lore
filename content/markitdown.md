# markitdown

Convert various files (PDF, Office, images, audio) to Markdown.

## Usage
- `markitdown path/to/file.pdf` : convert a file and print to `stdout`
- `markitdown path/to/file.pdf -o path/to/output.md` : save output to a specific file
- `markitdown --keep-data-uris path/to/file.docx` : keep embedded base64 images instead of truncating them (such as from DOCX or HTML)
- `markitdown path/to/file.pdf -d -e endpoint_url` : convert a file using Azure Document Intelligence
- `markitdown -h` : display help

## Categories
documents, media, files

## Links
- More information: https://github.com/microsoft/markitdown
- tldr page: https://tldr.inbrowser.app/pages/common/markitdown
