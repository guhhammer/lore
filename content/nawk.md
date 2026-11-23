# nawk

The system's AWK program (here the mawk implementation): a small language for processing text columns.

## Usage
- `nawk '{ print $1 }' file.txt` : print the first column of every line
- `nawk -F, '{ print $2 }' data.csv` : split lines on commas and print the second field
- `nawk 'NR > 1' file.txt` : skip the first line
- `nawk '{ n++ } END { print n }' file.txt` : count the lines

## Categories
text, documents, files

## Links
- Documentation: https://www.gnu.org/soft‐
