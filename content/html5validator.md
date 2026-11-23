# html5validator

Validate HTML5.

## Usage
- `html5validator path/to/file` : validate a specific file
- `html5validator --root path/to/directory` : validate all HTML files in a specific directory
- `html5validator --show-warnings path/to/file` : show warnings as well as errors
- `html5validator --root path/to/directory --match "*.html *.php"` : match multiple files using a `glob` pattern
- `html5validator --root path/to/directory --blacklist "node_modules vendor"` : ignore specific directory names
- `html5validator --format gnu|xml|json|text path/to/file` : output the results in a specific format
- `html5validator --root path/to/directory --log debug|info|warning` : output the log at a specific verbosity level

## Categories
files, text

## Links
- More information: https://github.com/svenkreiss/html5validator
- tldr page: https://tldr.inbrowser.app/pages/common/html5validator
