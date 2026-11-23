# expand

Convert tabs to spaces.

## Usage
- `expand path/to/file` : convert tabs in each file to spaces, writing to `stdout`
- `expand` : convert tabs to spaces, reading from `stdin`
- `expand -i path/to/file` : do not convert tabs after non blanks
- `expand -t number path/to/file` : have tabs a certain number of characters apart, not 8
- `expand -t 1,4,6` : use a comma separated list of explicit tab positions

## Categories
files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/expand
