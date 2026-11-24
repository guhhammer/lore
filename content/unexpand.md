# unexpand

Convert spaces to tabs.

## Usage
- `unexpand path/to/file` : convert blanks in each file to tabs, writing to `stdout`
- `unexpand` : convert blanks to tabs, reading from `stdout`
- `unexpand -a path/to/file` : convert all blanks, instead of just initial blanks
- `unexpand --first-only path/to/file` : convert only leading sequences of blanks (overrides -a)
- `unexpand -t number path/to/file` : have tabs a certain number of characters apart, not 8 (enables -a)

## Categories
files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/unexpand
- Wikipedia: https://en.wikipedia.org/wiki/Unexpand
