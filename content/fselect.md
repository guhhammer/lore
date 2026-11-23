# fselect

Find files with SQL-like queries.

## Usage
- `fselect size, path from path/to/directory where name = '*.cfg' or name = '*.tmp'` : select full path and size from temporary or configuration files in a given directory
- `fselect path from path/to/directory where width = height` : find square images
- `fselect path from path/to/directory where genre = Rap and bitrate = 320 and mp3_year lt 2000` : find old-school rap 320kbps MP3 files
- `fselect size, path from path/to/directory limit 5 into json` : select only the first 5 results and output as JSON
- `fselect "MIN(size), MAX(size), AVG(size), SUM(size), COUNT(*) from path/to/directory"` : use SQL aggregate functions to calculate minimum, maximum, and average size of files in a directory

## Categories
files, databases

## Links
- More information: https://github.com/jhspetersson/fselect/blob/master/docs/usage.md
- tldr page: https://tldr.inbrowser.app/pages/common/fselect
