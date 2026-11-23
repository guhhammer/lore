# pg_combinebackup

Reconstruct a full (synthetic) PostgreSQL backup from an incremental backup chain. When specifying multiple backups order them from oldest to newest.

## Usage
- `pg_combinebackup path/to/full_backup path/to/incremental_backup -o path/to/output_directory` : combine a full and incremental backup into one synthetic full backup
- `pg_combinebackup -n path/to/full_backup path/to/incremental_backup -o path/to/output_directory` : simulate what would be done without creating files
- `pg_combinebackup -k path/to/full_backup path/to/incremental_backup -o path/to/output_directory` : use hard links instead of copying files (faster, same filesystem required)
- `pg_combinebackup --clone path/to/full_backup path/to/incremental_backup -o path/to/output_directory` : use file cloning (reflinks) for efficient copy if supported
- `pg_combinebackup --copy-file-range path/to/full_backup path/to/incremental_backup -o path/to/output_directory` : use the `copy_file_range` system call for efficient copying
- `pg_combinebackup path/to/backup1 path/to/backup2 ... -T /path/to/old_tablespace=/path/to/new_tablespace -o path/to/output_directory` : relocate a tablespace during reconstruction
- `pg_combinebackup -N path/to/backup1 path/to/backup2 ... -o path/to/output_directory` : disable fsync for faster but unsafe writes (testing only)
- `pg_combinebackup -d path/to/backup1 path/to/backup2 ... -o path/to/output_directory` : show detailed debug output

## Categories
files

## Links
- More information: https://www.postgresql.org/docs/current/app-pgcombinebackup.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_combinebackup
