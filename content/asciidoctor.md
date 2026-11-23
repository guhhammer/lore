# asciidoctor

Convert AsciiDoc files to a publishable format.

## Usage
- `asciidoctor path/to/file.adoc` : convert a specific `.adoc` file to HTML (the default output format)
- `asciidoctor -a stylesheet=path/to/stylesheet.css path/to/file.adoc` : convert a specific `.adoc` file to HTML and link a CSS stylesheet
- `asciidoctor -e path/to/file.adoc` : convert a specific `.adoc` file to embeddable HTML, removing everything except the body
- `asciidoctor -b pdf -r asciidoctor-pdf path/to/file.adoc` : convert a specific `.adoc` file to a PDF using the `asciidoctor-pdf` library

## Categories
files, documents, media

## Links
- More information: https://docs.asciidoctor.org/asciidoctor/latest/cli/man1/asciidoctor/
- tldr page: https://tldr.inbrowser.app/pages/common/asciidoctor
