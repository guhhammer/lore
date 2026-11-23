# mapfile

This command is an alias of `readarray`.

## Usage
- `readarray array_name` : interactively input lines into an array
- `readarray <path/to/file.txt array_name` : read lines from a file and insert them in an array
- `readarray <path/to/file.txt -t array_name` : remove trailing delimiters (newline by default)
- `readarray <path/to/file.txt -n n array_name` : copy at most `n` lines
- `readarray <path/to/file.txt -s n array_name` : skip the first `n` lines
- `readarray <path/to/file.txt -d delimiter array_name` : define a custom delimiter
- `help mapfile` : display help

## Categories
files, text, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/mapfile
