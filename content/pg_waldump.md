# pg_waldump

Display a human-readable rendering of the write-ahead log (WAL) of a PostgreSQL database cluster.

## Usage
- `pg_waldump start_segment` : display WAL records from a specific segment
- `pg_waldump start_segment end_segment` : display WAL records between two segments
- `pg_waldump start_segment end_segment -p path/to/directory` : specify the WAL file directory
- `pg_waldump start_segment end_segment -f` : follow new WAL entries as they arrive
- `pg_waldump start_segment end_segment -n count` : limit number of records shown
- `pg_waldump start_segment end_segment -z` : display summary statistics instead of individual records
- `pg_waldump start_segment end_segment -r rmgr_name` : filter by resource manager
- `pg_waldump -?` : display help

## Categories
desktop, files, monitoring

## Links
- More information: https://www.postgresql.org/docs/current/pgwaldump.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_waldump
