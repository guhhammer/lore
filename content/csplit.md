# csplit

Split a file into pieces. This generates files named `xx00`, `xx01`, and so on.

## Usage
- `csplit path/to/file 10` : split a file in two parts, starting the second one at line 10
- `csplit path/to/file 7 23` : split a file in three parts, starting the latter parts in lines 7 and 23
- `csplit path/to/file 5 {*}` : start a new part at every 5th line (will fail if number of lines is not divisible by 5)
- `csplit -k path/to/file 5 {*}` : start a new part at every 5th line, ignoring exact-division error
- `csplit path/to/file 5 -f prefix` : split a file above line 5 and use a custom prefix for the output files (default is `xx`)
- `csplit path/to/file /regex/` : split a file above the first line matching a `regex` pattern
- `csplit -k -f prefix path/to/file '/regex/' '{99}'` : split a file before each of the first 100 lines matching a `regex` and use a custom file prefix

## Categories
files, text

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/csplit
- Wikipedia: https://en.wikipedia.org/wiki/Csplit
