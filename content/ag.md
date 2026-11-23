# ag

The Silver Searcher. Like `ack`, but aims to be faster.

## Usage
- `ag string` : find files containing `string`, and print the line matches in context
- `ag string path/to/directory` : find files containing `string` in a specific directory
- `ag -l string` : find files containing `string`, but only list the filenames
- `ag -i -o STRING` : find files containing `STRING` case-insensitively, and print only the match, rather than the whole line
- `ag string -G file_name` : find `string` in files with a name matching `file_name`
- `ag '^ca(t|r)$'` : find files whose contents match a `regex`
- `ag -g string` : find files with a name matching `string`

## Categories
text, files

## Links
- More information: https://manned.org/ag
- tldr page: https://tldr.inbrowser.app/pages/common/ag
