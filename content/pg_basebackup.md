# pg_basebackup

Take a base backup of a running PostgreSQL cluster. Used for full or incremental backups, point-in-time recovery, or setting up replication standbys.

## Usage
- `pg_basebackup -h host -D path/to/backup_dir` : take a base backup from a remote PostgreSQL server
- `pg_basebackup -h host -D path/to/backup_dir -P` : take a backup with progress shown
- `pg_basebackup -D path/to/backup_dir -F t -z` : create a compressed backup (`gzip`) in `tar` format
- `pg_basebackup -D path/to/backup_dir -i path/to/old_manifest` : create an incremental backup using a previous manifest file
- `pg_basebackup -D path/to/backup_dir -R` : write a recovery configuration for setting up a standby
- `pg_basebackup -D path/to/backup_dir -T path/to/old_tablespace=path/to/new_tablespace` : relocate a tablespace during backup
- `pg_basebackup -D path/to/backup_dir -r 100M` : limit transfer rate to reduce server load
- `pg_basebackup -D path/to/backup_dir -X stream` : stream WAL logs while taking the backup

## Categories
files, network, containers

## Links
- More information: https://www.postgresql.org/docs/current/app-pgbasebackup.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_basebackup
