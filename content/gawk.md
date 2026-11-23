# gawk

GNU version of awk, a versatile programming language for working on files.

## Usage
- `gawk '{print $5}' path/to/file` : print the fifth column (a.k.a. field) in a space-separated file
- `gawk '/pattern/ {print $2}' path/to/file` : print the second column of the lines matching a pattern in a space-separated file
- `gawk -F ',' '{print $NF}' path/to/file` : print the last column of each line in a file, using a comma (instead of space) as a field separator
- `gawk '{s+=$1} END {print s}' path/to/file` : sum the values in the first column of a file and print the total
- `gawk 'NR%3==1' path/to/file` : print every third line starting from the first line
- `gawk '{if ($1 == "string1") print "Exact match"; else if ($1 ~ "string2") print "Partial match"; else print "No match"}' path/to/file` : print different values based on conditions
- `gawk '($10 >= min_value && $10 <= max_value)' path/to/file` : print all the lines which the 10th column value is between a min and a max
- `gawk 'BEGIN {FS=":";printf "%-20s %6s %25s\n", "Name", "UID", "Shell"} $4 >= 1000 {printf "%-20s %6d %25s\n", $1, $4, $7}' /etc/passwd` : print table of users with UID >=1000 with header and formatted output, using colon as separator (`%-20s` mean: 20 left-align string characters, `%6s` means: 6 right-align string characters)

## See also
- awk

## Categories
files, documents, text

## Links
- More information: https://www.gnu.org/software/gawk/manual/gawk.html
- tldr page: https://tldr.inbrowser.app/pages/common/gawk
