# weasyprint

Render HTML to PDF or PNG.

## Usage
- `weasyprint path/to/input.html path/to/output.pdf` : render an HTML file to PDF
- `weasyprint path/to/input.html path/to/output.png --stylesheet path/to/stylesheet.css` : render an HTML file to PNG, including an additional user stylesheet
- `weasyprint path/to/input.html path/to/output.pdf --verbose` : output additional debugging information when rendering
- `weasyprint path/to/input.html path/to/output.png --resolution 300` : specify a custom resolution when outputting to PNG
- `weasyprint path/to/input.html path/to/output.png --base-url url_or_filename` : specify a base URL for relative URLs in the input HTML file

## Categories
files, documents, media

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/weasyprint
