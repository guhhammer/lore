# zegrep

This command is an alias of `zgrep --extended-regexp`.

## Usage
- `zgrep pattern path/to/compressed_file` : grep a pattern in a compressed file (case-sensitive)
- `zgrep --context|--before-context|--after-context 3 pattern path/to/compressed_file` : print 3 lines of Context around, Before, or After each match
- `zgrep -i pattern path/to/compressed_file` : grep a pattern in a compressed file (case-insensitive)
- `zgrep -c pattern path/to/compressed_file` : output count of lines containing matched pattern in a compressed file
- `zgrep -v pattern path/to/compressed_file` : display the lines which don't have the pattern present (Invert the search function)
- `zgrep -e "pattern_1" -e "pattern_2" path/to/compressed_file` : grep a compressed file for multiple patterns
- `zgrep -E regex path/to/file` : use extended `regex` (supports `?`, `+`, `{}`, `()`, and `|`)

## Categories
text, files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/zegrep
