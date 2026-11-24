# sort

Sort lines of text files.

## Usage
- `sort path/to/file` : sort a file in ascending order
- `sort -r path/to/file` : sort a file in descending order
- `sort -f path/to/file` : sort a file in case-insensitive way
- `sort -n path/to/file` : sort a file using numeric rather than alphabetic order
- `sort -t : -k 3n /etc/passwd` : sort `/etc/passwd` by the 3rd field onward of each line numerically, using `:` as a field separator
- `sort -t : -k 3,3n -k 4,4g /etc/passwd` : sort `/etc/passwd` numerically by the 3rd field, and when items in the 3rd field are equal, sort by the 4th field by numbers with exponents
- `sort -u path/to/file` : sort a file preserving only unique lines
- `sort -o path/to/output_file path/to/input_file` : sort a file, printing the output to the specified output file (can be used to sort a file in-place)

## Categories
text

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sort-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/sort
- Wikipedia: https://en.wikipedia.org/wiki/Sort_(Unix)
