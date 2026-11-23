# highlight

Output syntax-highlighted source code to a variety of formats.

## Usage
- `highlight -O html -s theme_name -S language path/to/source_code` : produce a complete HTML document from a source code file
- `highlight -O html -f -S language source_file` : produce an HTML fragment, suitable for inclusion in a larger document
- `highlight -O html --inline-css -S language source_file` : inline the CSS styling in every tag
- `highlight --list-scripts langs|themes|plugins` : list all supported languages, themes, or plugins
- `highlight -O html --print-style -s theme_name -S language --stdout` : print a CSS stylesheet for a theme

## Categories
development, documents

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/highlight
