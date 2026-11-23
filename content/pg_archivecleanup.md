# pg_archivecleanup

Remove old WAL archive files in PostgreSQL.

## Usage
- `pg_archivecleanup path/to/archive path/to/walfile` : clean an archive directory up to a given WAL file
- `pg_archivecleanup -n path/to/archive path/to/walfile` : simulate removing files without actually doing it
- `pg_archivecleanup -x extension path/to/archive path/to/walfile` : strip a file extension before deciding deletion
- `pg_archivecleanup -b path/to/archive path/to/walfile` : remove backup history files too
- `pg_archivecleanup -d path/to/archive path/to/walfile` : enable debug logging output

## Categories
files, archives

## Links
- More information: https://www.postgresql.org/docs/current/pgarchivecleanup.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_archivecleanup
