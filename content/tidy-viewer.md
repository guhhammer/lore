# tidy-viewer

A data pretty-printer with column styling for CSV, TSV, PSV, Parquet, and Arrow IPC files. Commonly aliased as `tv`.

## Usage
- `tidy-viewer path/to/file.csv` : pretty-print the first 25 rows of a CSV file
- `cat path/to/file.csv | tidy-viewer` : pretty-print data piped from `stdin`
- `tidy-viewer -s '|' path/to/file.psv` : pretty-print a file with a custom separator (e.g. a pipe)
- `tidy-viewer -c 3 path/to/file.csv` : display data with a specific color palette (1: nord, 2: one_dark, 3: gruvbox, 4: dracula, 5: solarized light)
- `tidy-viewer -af path/to/file.csv | less -RS` : print all rows in color, paging through the output
- `tidy-viewer -h` : display help
- `tidy-viewer -V` : display version

## Categories
files, text, documents

## Links
- More information: https://github.com/alexhallam/tv#help
- tldr page: https://tldr.inbrowser.app/pages/common/tidy-viewer
