# bat

Print and concatenate files. A `cat` clone with syntax highlighting and Git integration.

## Usage
- `bat path/to/file1 path/to/file2 ...` : pretty print the contents of one or more files to `stdout`
- `bat path/to/file1 path/to/file2 ... >path/to/target_file` : concatenate several files into the target file
- `bat -pp path/to/file` : remove decorations and disable paging
- `bat -H 10|5:10|:10|10:|10:+5 path/to/file` : highlight a specific line or a range of lines with a different background color
- `bat -A path/to/file` : show non-printable characters like space, tab, or newline
- `bat -n path/to/file` : remove all decorations except line numbers in the output
- `bat -l json path/to/file.json` : syntax highlight a JSON file by explicitly setting the language
- `bat -L` : display all supported languages

## See also
- cat

## Categories
files, text, documents

## Links
- More information: https://manned.org/bat
- tldr page: https://tldr.inbrowser.app/pages/common/bat
