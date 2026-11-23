# ugrep

Ultra fast search tool with query TUI.

## Usage
- `ugrep -Q` : start a query TUI to search files in the current directory recursively (`<Ctrl z>` for help)
- `ugrep "search_pattern"` : search the current directory recursively for files containing a `regex` search pattern
- `ugrep -n "search_pattern" path/to/file_or_directory` : search in a specific file or in all files in a specific directory, showing line numbers of matches
- `ugrep -l "search_pattern"` : search in all files in the current directory recursively and print the name of each matching file
- `ugrep -Z3 "search_pattern"` : fuzzy search files with up to 3 extra, missing, or mismatching characters in the pattern
- `ugrep -z "search_pattern"` : also search compressed files, Zip, and `.tar` archives recursively
- `ugrep -g "glob_pattern" "search_pattern"` : search only files whose filenames match a specific `glob` pattern
- `ugrep -t cpp "search_pattern"` : search only C++ source files (use `--file-type=list` to list all file types)

## Categories
files, text, databases

## Links
- More information: https://github.com/Genivia/ugrep#man-page
- tldr page: https://tldr.inbrowser.app/pages/common/ugrep
