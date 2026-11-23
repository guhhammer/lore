# csvgrep

Filter CSV rows with string and pattern matching. Included in csvkit.

## Usage
- `csvgrep -c 1 -m string_to_match data.csv` : find rows that have a certain string in column 1
- `csvgrep -c 3,4 -r regex data.csv` : find rows in which columns 3 or 4 match a certain `regex`
- `csvgrep -i -c name -m "John Doe" data.csv` : find rows in which the "name" column does NOT include the string "John Doe"

## Categories
text

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/csvgrep.html
- tldr page: https://tldr.inbrowser.app/pages/common/csvgrep
