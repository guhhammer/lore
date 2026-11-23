# fd

Find entries in the filesystem.

## Usage
- `fd "regex"` : recursively find files matching a specific pattern in the current directory
- `fd "regex" path/to/directory` : find files in a specific directory
- `fd -e txt` : find files with a specific extension
- `fd "regex" -t d` : find only directories matching a specific pattern
- `fd "regex" -HI` : include ignored and hidden files in the search
- `fd "regex" -E glob` : exclude files that match a specific `glob` pattern
- `fd "regex" -x command` : execute a command on each search result returned
- `fd "regex" -d 1` : find files only in the current directory

## See also
- find
- regex

## Categories
text, files, disks

## Links
- More information: https://github.com/sharkdp/fd#command-line-options
- tldr page: https://tldr.inbrowser.app/pages/common/fd
