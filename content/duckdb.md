# duckdb

Client for DuckDB, an in-process analytical SQL engine.

## Usage
- `duckdb` : start an interactive shell with a transient in-memory database
- `duckdb path/to/dbfile` : start an interactive shell on a database file. If the file does not exist, a new database is created
- `duckdb -c "SELECT * FROM 'data_source.ext'"` : query a CSV, JSON, or Parquet file using SQL
- `duckdb data_source.ext -c " SELECT * FROM file "` : directly query a CSV, JSON, or Parquet file using the `file` view
- `duckdb -f path/to/script.sql` : run an SQL script
- `duckdb path/to/dbfile -cmd "SELECT DISTINCT * FROM tbl"` : run query on database file and keep the interactive shell open
- `cat path/to/source.csv | duckdb -c "COPY (FROM read_csv('/dev/stdin')) TO '/dev/stdout' WITH (FORMAT CSV, HEADER)"` : read CSV from `stdin` and write CSV to `stdout`
- `duckdb -ui` : start the DuckDB UI, a web interface with notebooks

## Categories
databases, files, text

## Links
- More information: https://duckdb.org/docs/stable/clients/cli/arguments
- tldr page: https://tldr.inbrowser.app/pages/common/duckdb
