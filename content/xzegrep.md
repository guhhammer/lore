# xzegrep

This command is an alias of `xzgrep --extended-regexp`.

## Usage
- `xzgrep "search_pattern" path/to/file` : search for a pattern within a file
- `xzgrep -F "exact_string" path/to/file` : search for an exact string (disables `regex`)
- `xzgrep -n "search_pattern" path/to/file` : search for a pattern in all files showing line numbers of matches
- `xzgrep --context|--before-context|--after-context 3 "search_pattern" path/to/file` : print 3 lines of Context around, Before, or After each match
- `xzgrep -H -n --color=always "search_pattern" path/to/file` : print file name and line number for each match with color output
- `xzgrep -o "search_pattern" path/to/file` : search for lines matching a pattern, printing only the matched text
- `xzgrep -E -i "search_pattern" path/to/file` : use extended `regex` (supports `?`, `+`, `{}`, `()`, and `|`), in case-insensitive mode

## Categories
files, text, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/xzegrep
