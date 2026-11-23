# textql

Execute SQL against structured text like CSV or TSV files.

## Usage
- `textql -sql "SELECT * FROM filename" path/to/filename.csv` : print the lines in the specified CSV file that match an SQL query to `stdout`
- `textql -dlm=tab -sql "SELECT * FROM filename" path/to/filename.tsv` : query a TSV file
- `textql -dlm=delimiter -header -sql "SELECT * FROM filename" path/to/filename.csv` : query file with header row
- `cat path/to/file | textql -sql "SELECT * FROM stdin"` : read data from `stdin`
- `textql -header -sql "SELECT * FROM path/to/file1 JOIN file2 ON path/to/file1.c1 = file2.c1 LIMIT 10" -output-header path/to/file1.csv path/to/file2.csv` : join two files on a specified common column
- `textql -output-dlm=delimiter -output-header -sql "SELECT column AS alias FROM filename" path/to/filename.csv` : format output using an output delimiter with an output header line

## Categories
text, databases, files

## Links
- More information: https://github.com/dinedal/textql#usage
- tldr page: https://tldr.inbrowser.app/pages/common/textql
