# tw

View and query tabular data files using Tabiew.

## Usage
- `tw path/to/file1 path/to/file2 ...` : open one or more files, detecting the format based on the file extension
- `tw path/to/file -f csv|tsv|parquet|sqlite|excel|...` : open a file, overriding the format detection
- `tw path/to/file.csv --separator '|'` : open a delimiter-separated file with a custom separator
- `tw path/to/file.csv --no-header` : open a delimiter-separated file without a header row
- `curl -s https://example.com/data.csv | tw` : read tabular data from `stdin`
- `tw --multiparts path/to/part1.csv path/to/part2.csv ...` : open files with identical columns and concatenate them vertically into a single table

## Categories
files, text, databases

## Links
- More information: https://github.com/shshemi/tabiew#usage
- tldr page: https://tldr.inbrowser.app/pages/common/tw
