# gnucut

This command is an alias of GNU `cut`.

## Usage
- `command | cut -c 5` : print the fifth character on each line
- `cut -c 5-10 path/to/file` : print the fifth to tenth character of each line of the specified file
- `cut -f 2,6 path/to/file` : split each line in a file by a delimiter into fields and print fields two and six (default delimiter is `TAB`)
- `command | cut -d "delimiter" -f 2-` : split each line by the specified delimiter and print all from the second field onward
- `command | cut -d " " -f -3` : use space as a delimiter and print only the first 3 fields
- `command | cut -d ":" -f 1 -s` : only print lines that contain the delimiter
- `find . -print0 | cut -z -d "/" -f 2` : print specific fields of lines that use `NUL` to terminate lines instead of newlines

## Categories
text, documents, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnucut
