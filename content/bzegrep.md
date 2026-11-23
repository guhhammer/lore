# bzegrep

This command is an alias of `bzgrep --extended-regexp`.

## Usage
- `bzgrep "search_pattern" path/to/file` : search for a pattern within a compressed file
- `bzgrep -r "search_pattern" path/to/tar_file` : recursively search files in a `bzip2` compressed `.tar` archive for a pattern
- `bzgrep --context|--before-context|--after-context 3 "search_pattern" path/to/file` : print 3 lines of Context around, Before, or After each match
- `bzgrep -H -n "search_pattern" path/to/file` : print file name and line number for each match
- `bzgrep -o "search_pattern" path/to/file` : search for lines matching a pattern, printing only the matched text
- `cat path/to/bz_compressed_file | bzgrep -v "search_pattern"` : search `stdin` for lines that do not match a pattern
- `bzgrep -E -i "search_pattern" path/to/file` : use extended `regex` (supports `?`, `+`, `{}`, `()`, and `|`), in case-insensitive mode

## Categories
files, text, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/bzegrep
