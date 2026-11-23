# qsv

Blazing-fast toolkit for querying, slicing, sorting, analyzing, filtering, transforming, and validating tabular data.

## Usage
- `qsv count path/to/file.csv` : count the rows in a CSV file
- `qsv select column1,column2,... path/to/file.csv` : select specific columns from a CSV file
- `qsv search regex_or_text path/to/file.csv` : filter rows containing a `regex` or literal text
- `qsv sort -s column1,column2,... path/to/file.csv` : sort a CSV file by one or more columns
- `qsv stats path/to/file.csv` : compute summary statistics and infer data types for each column
- `qsv --help` : display help
- `qsv --version` : display version

## Categories
files, text

## Links
- More information: https://qsv.dathere.com/
- tldr page: https://tldr.inbrowser.app/pages/common/qsv
