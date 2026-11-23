# pg_verifybackup

Verify the integrity of a base backup of a PostgreSQL cluster.

## Usage
- `pg_verifybackup path/to/backup` : verify a backup stored in a specific directory
- `pg_verifybackup -P path/to/backup` : verify a backup showing progress information
- `pg_verifybackup -e path/to/backup` : verify a backup and exit immediately on first error
- `pg_verifybackup -i path/to/file_or_directory path/to/backup` : verify a backup ignoring specific files or directories
- `pg_verifybackup -m path/to/backup_manifest path/to/backup` : verify a backup with a manifest file in a different location
- `pg_verifybackup -?` : display help

## Categories
files, containers

## Links
- More information: https://www.postgresql.org/docs/current/app-pgverifybackup.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_verifybackup
