# pg_resetwal

Reset the write-ahead log and other control information of a PostgreSQL database cluster.

## Usage
- `pg_resetwal -D path/to/data` : reset the WAL and control information for a specific data directory
- `pg_resetwal -D path/to/data -n` : simulate resetting without actually doing it
- `pg_resetwal -D path/to/data -f` : force the WAL and control information reset
- `pg_resetwal -?` : display help
- `pg_resetwal -V` : display version

## Categories
files, containers, databases

## Links
- More information: https://www.postgresql.org/docs/current/app-pgresetwal.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_resetwal
