# nl

Number lines from a file or from `stdin`.

## Usage
- `nl path/to/file` : number non-blank lines in a file
- `command | nl -` : read from `stdin`
- `nl -b a|n path/to/file` : number all body lines including blank lines or do not number body lines
- `nl -b p'FooBar[0-9]' path/to/file` : number only the body lines that match a basic `regex` (BRE) pattern
- `nl -i increment path/to/file` : use a specific increment for line numbering
- `nl -n rz|ln|rn` : specify the line numbering format to right or left justified, keeping leading zeros or not
- `nl -w col_width path/to/file` : specify the line numbering's width (6 by default)
- `nl -s separator path/to/file` : use a specific string to separate the line numbers from the lines (`TAB` by default)

## Categories
files, text

## Links
- More information: https://manned.org/nl.1p
- tldr page: https://tldr.inbrowser.app/pages/common/nl
