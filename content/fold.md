# fold

Wrap each line in an input file to fit a specified width and print it to `stdout`.

## Usage
- `fold path/to/file` : wrap each line to default width (80 characters)
- `fold -w30 path/to/file` : wrap each line to width "30"
- `fold -w5 -s path/to/file` : wrap each line to width "5" and break the line at spaces (puts each space separated word in a new line, words with length > 5 are wrapped)

## Categories
files, text, documents

## Links
- More information: https://manned.org/fold.1p
- tldr page: https://tldr.inbrowser.app/pages/common/fold
- Wikipedia: https://en.wikipedia.org/wiki/Fold_(Unix)
