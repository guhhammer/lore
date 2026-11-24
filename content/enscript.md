# enscript

Convert text files to PostScript, HTML, RTF, ANSI, and overstrikes.

## Usage
- `enscript path/to/input_file -o path/to/output_file` : generate a PostScript file from a text file
- `enscript path/to/input_file -w html|rtf|... -o path/to/output_file` : generate a file in a different language than PostScript
- `enscript path/to/input_file --columns num -r -o path/to/output_file` : generate a PostScript file with a landscape layout, splitting the page into columns (maximum 9)
- `enscript --help-highlight` : display available syntax highlighting languages and file formats
- `enscript path/to/input_file --color 1 -E language -o path/to/output_file` : generate a PostScript file with syntax highlighting and color for a specified language

## Categories
files, text

## Links
- More information: https://manned.org/enscript
- tldr page: https://tldr.inbrowser.app/pages/common/enscript
- Wikipedia: https://en.wikipedia.org/wiki/Enscript
