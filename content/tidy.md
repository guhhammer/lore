# tidy

Clean up and pretty print HTML, XHTML, and XML files. Note: `tidy` cannot preserve original indentation.

## Usage
- `tidy path/to/file.html` : pretty print an HTML file
- `tidy -i y -w 100 -o path/to/output.html path/to/file.html` : enable indentation, wrapping lines in 100, saving to `output.html`
- `tidy -config path/to/configuration -m path/to/file.html` : modify an HTML file in-place using a configuration file

## Categories
files, documents, text

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/tidy
