# ctags

Generate an index (or tag) file of language objects found in source files for many popular programming languages.

## Usage
- `ctags path/to/file` : generate tags for a single file and output them to a file named "tags" in the current directory, overwriting the file if it exists
- `ctags -f path/to/file *` : generate tags for all files in the current directory and output them to a specific file, overwriting the file if it exists
- `ctags --recurse` : generate tags for all files in the current directory and all subdirectories
- `ctags --fields=+ne --output-format=json path/to/file` : generate tags for a single file and output them with start line number and end line number in JSON format

## Categories
files, text, development

## Links
- More information: https://docs.ctags.io/en/latest/man/ctags.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/ctags
