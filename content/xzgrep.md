# xzgrep

Search files possibly compressed with `xz`, `lzma`, `gzip`, `bzip2`, `lzop`, or `zstd` using `regex`.

## Usage
- `xzgrep "search_pattern" path/to/file` : search for a pattern within a file
- `xzgrep -F "exact_string" path/to/file` : search for an exact string (disables `regex`)
- `xzgrep -n "search_pattern" path/to/file` : search for a pattern in all files showing line numbers of matches
- `xzgrep --context|--before-context|--after-context 3 "search_pattern" path/to/file` : print 3 lines of Context around, Before, or After each match
- `xzgrep -H -n --color=always "search_pattern" path/to/file` : print file name and line number for each match with color output
- `xzgrep -o "search_pattern" path/to/file` : search for lines matching a pattern, printing only the matched text
- `xzgrep -E -i "search_pattern" path/to/file` : use extended `regex` (supports `?`, `+`, `{}`, `()`, and `|`), in case-insensitive mode

## See also
- grep

## Categories
files, text, archives

## Links
- More information: https://manned.org/xzgrep
- tldr page: https://tldr.inbrowser.app/pages/common/xzgrep
