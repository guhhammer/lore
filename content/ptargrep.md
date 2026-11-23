# ptargrep

Find `regex` patterns in `.tar` archive files.

## Usage
- `ptargrep "search_pattern" path/to/file1 path/to/file2 ...` : search for a pattern within one or more `.tar` archives
- `ptargrep -b "search_pattern" path/to/file` : extract to the current directory using the basename of the file from the archive
- `ptargrep -i "search_pattern" path/to/file` : search for a case-insensitive pattern matching within a `.tar` archive

## Categories
archives, files, text

## Links
- More information: https://manned.org/ptargrep
- tldr page: https://tldr.inbrowser.app/pages/common/ptargrep
