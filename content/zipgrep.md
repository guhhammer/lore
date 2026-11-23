# zipgrep

Find patterns in files in a Zip archive using extended `regex` (supports `?`, `+`, `{}`, `()`, and `|`).

## Usage
- `zipgrep "search_pattern" path/to/file.zip` : search for a pattern within a Zip archive
- `zipgrep -H -n "search_pattern" path/to/file.zip` : print file name and line number for each match
- `zipgrep -v "search_pattern" path/to/file.zip` : search for lines that do not match a pattern
- `zipgrep "search_pattern" path/to/file.zip path/to/included_file1 path/to/included_file2 ...` : include only specific files inside a Zip archive in the search
- `zipgrep "search_pattern" path/to/file.zip -x path/to/excluded_file1 path/to/excluded_file2 ...` : exclude files inside a Zip archive from search

## Categories
archives, files, text

## Links
- More information: https://manned.org/zipgrep
- tldr page: https://tldr.inbrowser.app/pages/common/zipgrep
