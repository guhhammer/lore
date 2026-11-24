# pandoc

Convert documents between various formats.

## Usage
- `pandoc path/to/input.md -o path/to/output.pdf` : convert a Markdown file to PDF using `pdflatex` (the formats are determined by file extensions)
- `command | pandoc -f input_format --pdf-engine tectonic|weasyprint|typst|... -o path/to/output.pdf` : convert the output from another command to PDF, using a specific PDF engine
- `pandoc path/to/input.md -s -o path/to/output.html` : convert to a standalone file with the appropriate headers/footers (for LaTeX, HTML, etc.)
- `pandoc -f docx|... path/to/input -t pdf|... -o path/to/output` : manually specify formats (overriding automatic format detection using the filename extension, or when there is no extension)
- `pandoc path/to/input -L path/to/filter.lua -o path/to/output` : transform a document using a Lua script (see <https://pandoc.org/lua-filters.html> for more information)
- `pandoc -f html -t markdown https://example.com` : convert a remote HTML file to markdown and print the result to `stdout`
- `pandoc --list-input-formats` : list all supported input formats
- `pandoc --list-output-formats` : list all supported output formats

## Categories
files, documents, media

## Links
- More information: https://pandoc.org/MANUAL.html
- tldr page: https://tldr.inbrowser.app/pages/common/pandoc
- Wikipedia: https://en.wikipedia.org/wiki/Pandoc
