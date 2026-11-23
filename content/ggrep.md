# ggrep

This command is an alias of GNU `grep`.

## Usage
- `grep "search_pattern" path/to/file1 path/to/file2 ...` : search for a pattern within files
- `grep -F "exact_string" path/to/file` : search for an exact string (disables `regex`es)
- `grep -rI "search_pattern" path/to/directory` : search for a pattern in all files recursively in a directory, ignoring binary files
- `grep --context|--before-context|--after-context 3 "search_pattern" path/to/file` : print 3 lines of Context around, Before, or After each match
- `grep -Hn --color=always "search_pattern" path/to/file` : print file name and line number for each match with color output
- `grep -o "search_pattern" path/to/file` : print only the matched text
- `cat path/to/file | grep -v "search_pattern"` : read data from `stdin` and do not print lines that match a pattern
- `grep -Ei "search_pattern" path/to/file` : use extended `regex`es (supports `?`, `+`, `{}`, `()`, and `|`), in case-insensitive mode

## Categories
files, text, documents, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/ggrep
