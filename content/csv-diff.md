# csv-diff

View differences between two CSV, TSV, or JSON files.

## Usage
- `csv-diff path/to/file1.csv path/to/file2.csv --key column_name` : display a human-readable summary of differences between files using a specific column as a unique identifier
- `csv-diff path/to/file1.csv path/to/file2.csv --key column_name --show-unchanged` : display a human-readable summary of differences between files that includes unchanged values in rows with at least one change
- `csv-diff path/to/file1.csv path/to/file2.csv --key column_name --json` : display a summary of differences between files in JSON format using a specific column as a unique identifier

## Categories
text, version-control, files

## Links
- More information: https://github.com/simonw/csv-diff
- tldr page: https://tldr.inbrowser.app/pages/common/csv-diff
