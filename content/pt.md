# pt

Platinum Searcher. A code search tool similar to `ag`.

## Usage
- `pt pattern` : find files containing a pattern and print the files with highlighted matches
- `pt -c pattern` : find files containing a pattern and display count of matches in each file
- `pt -wi pattern` : find files containing a pattern as a whole word and ignore its case
- `pt -G='\.ext$' pattern` : find a pattern in files with a given extension using a `regex`
- `pt --depth=2 -e '^ba[rz]*$'` : find files whose contents match the `regex`, up to 2 directories deep

## Categories
text

## Links
- More information: https://github.com/monochromegane/the_platinum_searcher
- tldr page: https://tldr.inbrowser.app/pages/common/pt
