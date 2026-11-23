# keep-header

Keep first line untouched by a command, passing it directly to `stdout`.

## Usage
- `keep-header path/to/file -- sort` : sort a file and keep the first line at the top
- `keep-header path/to/file -- command` : output first line directly to `stdout`, passing the remainder of the file through the specified command
- `cat path/to/file | keep-header -- command` : read from `stdin`, sorting all except the first line
- `keep-header path/to/file -- grep pattern` : grep a file, keeping the first line regardless of the search pattern

## Categories
files, text

## Links
- More information: https://github.com/eBay/tsv-utils#keep-header
- tldr page: https://tldr.inbrowser.app/pages/common/keep-header
