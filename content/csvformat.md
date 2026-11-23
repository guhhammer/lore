# csvformat

Convert a CSV file to a custom output format. Included in csvkit.

## Usage
- `csvformat -T data.csv` : convert to a tab-delimited file (TSV)
- `csvformat -D "custom_character" data.csv` : convert delimiters to a custom character
- `csvformat -M "\r\n" data.csv` : convert line endings to carriage return (^M) + line feed
- `csvformat -U 0 data.csv` : minimize use of quote characters
- `csvformat -U 1 data.csv` : maximize use of quote characters

## Categories
text, files

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/csvformat.html
- tldr page: https://tldr.inbrowser.app/pages/common/csvformat
