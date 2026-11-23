# rdfind

Find files with duplicate content and get rid of them.

## Usage
- `rdfind -dryrun true path/to/directory` : identify all duplicates in a given directory and output a summary
- `rdfind -makehardlinks true path/to/directory` : replace all duplicates with hardlinks
- `rdfind -makesymlinks true path/to/directory` : replace all duplicates with symlinks/soft links
- `rdfind -deleteduplicates true -ignoreempty false path/to/directory` : delete all duplicates and do not ignore empty files

## Categories
files

## Links
- More information: https://rdfind.pauldreik.se/rdfind.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/rdfind
