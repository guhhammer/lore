# mawk

A fast implementation of AWK, the small language for picking apart and summing columns of text.

## Usage
- `mawk '{ print $1 }' file.txt` : print the first column of every line
- `mawk -F: '{ print $1 }' /etc/passwd` : use a colon as the field separator
- `mawk '{ s += $2 } END { print s }' file.txt` : add up the second column
- `mawk '/error/' file.log` : print only the lines that match a pattern
- `mawk -f script.awk file.txt` : run a program stored in a file

## Categories
text, documents, files

## Links
- Documentation: https://pubs.opengroup.org/onlinepubs/9799919799/utilities/awk.html
