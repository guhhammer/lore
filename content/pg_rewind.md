# pg_rewind

Synchronize a PostgreSQL data directory with another data directory that was forked from it.

## Usage
- `pg_rewind -D path/to/target_data --source-pgdata path/to/source_data` : synchronize target directory with source directory
- `pg_rewind -D path/to/target_data --source-server connstr` : synchronize target with source server using connection string
- `pg_rewind -D path/to/target_data --source-pgdata path/to/source_data -n` : simulate a synchronization without actually doing it
- `pg_rewind -D path/to/target_data --source-pgdata path/to/source_data -P` : show progress during synchronization
- `pg_rewind -?` : display help

## Categories
files

## Links
- More information: https://www.postgresql.org/docs/current/app-pgrewind.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_rewind
