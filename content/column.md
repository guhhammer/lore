# column

Format `stdin` or a file into multiple columns. Columns are filled before rows; the default separator is a whitespace.

## Usage
- `printf "header1 header2\nbar foo\n" | column -c 30` : format the output of a command for a 30 characters wide display
- `printf "header1 header2\nbar foo\n" | column -t` : split columns automatically and auto-align them in a tabular format
- `printf "header1,header2\nbar,foo\n" | column -t -s ,` : specify the column delimiter character for the `--table` option (e.g. "," for CSV) (defaults to whitespace)
- `printf "header1\nbar\nfoobar\n" | column -c 30 -x` : fill rows before filling columns

## Categories
text, files

## Links
- More information: https://manned.org/column
- tldr page: https://tldr.inbrowser.app/pages/linux/column
