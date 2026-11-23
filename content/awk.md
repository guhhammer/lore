# awk

A versatile programming language for working on files. Note: Different implementations of AWK often make this a symlink of their binary.

## Usage
- `awk '{print $5}' path/to/file` : print the fifth column (a.k.a. field) in a space-separated file
- `awk '/string1/ {print $2}' path/to/file` : print the second column of the lines containing "string1" in a space-separated file
- `awk -F ',' '{print $NF}' path/to/file` : print the last column of each line in a file, using a comma (instead of space) as a field separator
- `awk '{s+=$1} END {print s}' path/to/file` : sum the values in the first column of a file and print the total
- `awk 'NR%3==1' path/to/file` : print every third line starting from the first line
- `awk '{if ($1 == "string1") print "Exact match string1"; else if ($1 ~ "string2") print "Partial match string2"; else print "string3"}' path/to/file` : print different values based on conditions
- `awk '($10 >= min_value && $10 <= max_value)' path/to/file` : print all the lines which the 10th column value is between a min and a max
- `awk 'BEGIN {FS=":";printf "%-20s %6s %25s\n", "Name", "UID", "Shell"} $4 >= 1000 {printf "%-20s %6d %25s\n", $1, $4, $7}' /etc/passwd` : print table of users with UID >=1000 with header and formatted output, using colon as separator (`%-20s` mean: 20 left-align string characters, `%6s` means: 6 right-align string characters)

## See also
- gawk

## Categories
text

## Links
- More information: https://github.com/onetrueawk/awk
- tldr page: https://tldr.inbrowser.app/pages/common/awk
