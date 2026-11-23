# csvtool

Utility to filter and extract data from CSV formatted sources.

## Usage
- `csvtool -c 2 path/to/file.csv` : extract the second column from a CSV file
- `csvtool -c 2,4 path/to/file.csv` : extract the second and fourth columns from a CSV file
- `csvtool -c 2 -s '^String$' path/to/file.csv` : extract lines from a CSV file where the second column exactly matches `String`
- `csvtool -c 2 -s '^Bar' path/to/file.csv` : extract lines from a CSV file where the second column starts with `Bar`
- `csvtool -c 2 -s 'Baz$' path/to/file.csv | csvtool -e -c 3,6` : find lines in a CSV file where the second column ends with `Baz` and then extract the third and sixth columns

## Categories
text, files, archives

## Links
- More information: https://github.com/maroofi/csvtool
- tldr page: https://tldr.inbrowser.app/pages/common/csvtool
