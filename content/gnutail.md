# gnutail

This command is an alias of GNU `tail`.

## Usage
- `tail path/to/file` : show last 10 lines in a file
- `tail path/to/file1 path/to/file2 ...` : show last 10 lines of multiple files
- `tail -5 path/to/file` : show last 5 lines in file
- `tail -n +count path/to/file` : print a file from a specific line number
- `tail -c count path/to/file` : print a specific count of bytes from the end of a given file
- `tail -f path/to/file` : print the last lines of a given file and keep reading it until `<Ctrl c>`
- `tail -F path/to/file` : keep reading file until `<Ctrl c>`, even if the file is inaccessible
- `tail -n count -s seconds -f path/to/file` : show last `count` lines in a file and refresh every `seconds` seconds

## Categories
files, text, documents

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnutail
