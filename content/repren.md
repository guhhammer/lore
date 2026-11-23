# repren

Multi-pattern string replacement and file renaming tool.

## Usage
- `repren -n --rename --literal --from 'find_string' --to 'replacement_string' *.png` : simulate renaming a directory of PNGs with a literal string replacement
- `repren --rename -n --from 'regex' --to 'replacement_string' *.jpg *.jpeg` : simulate renaming a directory of JPEGs with a `regex`
- `repren --from '([0-9]+) example_string' --to 'replacement_string \1' *.csv` : do a find-and-replace on the contents of a directory of CSV files
- `repren -p path/to/patfile.ext --full *.txt` : do both a find-and-replace and a rename operation at the same time, using a pattern file
- `repren --rename -i -p path/to/patfile.ext *` : do a case-insensitive rename

## Categories
files, text

## Links
- More information: https://github.com/jlevy/repren
- tldr page: https://tldr.inbrowser.app/pages/common/repren
