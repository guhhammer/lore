# docling

Convert documents (PDF, DOCX, PPTX, HTML, images) to Markdown, JSON, or HTML using AI layout analysis and OCR.

## Usage
- `docling path/to/file.pdf` : convert a file to Markdown in the current directory
- `docling path/to/file.pdf --output path/to/directory` : save the converted output to a specific directory
- `docling --image-export-mode placeholder|referenced path/to/file.pdf` : replace images with placeholders, or export them as separate PNG files referenced from the output
- `docling --to json|html|md|text path/to/file.pdf` : export the document structure and tables to a specific format
- `docling --no-ocr path/to/file.pdf` : skip OCR to convert native digital PDFs much faster
- `docling -h` : display help

## Categories
files, documents, media

## Links
- More information: https://github.com/docling-project/docling
- tldr page: https://tldr.inbrowser.app/pages/common/docling
