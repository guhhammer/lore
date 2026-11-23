# pg_test_fsync

Determine the fastest wal_sync_method on your system.

## Usage
- `pg_test_fsync` : run the default fsync benchmark (5 seconds)
- `pg_test_fsync -s seconds` : specify a custom test duration
- `pg_test_fsync -f path/to/file` : use a specific filename (it must be in same filesystem that the pg_wal directory is or will be placed in)

## Categories
files

## Links
- More information: https://www.postgresql.org/docs/current/pgtestfsync.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_test_fsync
