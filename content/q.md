# q

Execute SQL-like queries on CSV and TSV files.

## Usage
- `q -d ',' "SELECT * from path/to/file"` : query a CSV file by specifying the delimiter as ','
- `q -t "SELECT * from path/to/file"` : query a TSV file
- `q -d delimiter -H "SELECT * from path/to/file"` : query file with header row
- `output | q "select * from -"` : read data from `stdin`; '-' in the query represents the data from `stdin`
- `q "SELECT * FROM path/to/file f1 JOIN path/to/other_file f2 ON (f1.c1 = f2.c1)"` : join two files (aliased as `f1` and `f2` in the example) on column `c1`, a common column
- `q -D delimiter -O "SELECT column as alias from path/to/file"` : format output using an output delimiter with an output header line (Note: Command will output column names based on the input file header or the column aliases overridden in the query)

## Categories
files, text, databases

## Links
- More information: https://harelba.github.io/q/
- tldr page: https://tldr.inbrowser.app/pages/common/q
- Wikipedia: https://en.wikipedia.org/wiki/Q_(software)
