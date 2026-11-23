# look

Display lines beginning with a prefix in a sorted file. Note: The lines in the file must be sorted.

## Usage
- `look prefix path/to/file` : search for lines beginning with a specific prefix in a specific file
- `look -f -d prefix path/to/file` : case-insensitively search only on alphanumeric characters
- `look -t ,` : specify a string termination character (space by default)
- `look prefix` : search in `/usr/share/dict/words` (`--alphanum` and `--ignore-case` are assumed)

## See also
- grep
- sort

## Categories
files, text, desktop

## Links
- More information: https://man.openbsd.org/look
- tldr page: https://tldr.inbrowser.app/pages/common/look
