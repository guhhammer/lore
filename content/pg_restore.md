# pg_restore

Restore a PostgreSQL database from an archive file created by pg_dump.

## Usage
- `pg_restore -d db_name archive_file.dump` : restore an archive into an existing database
- `pg_restore -U username -d db_name archive_file.dump` : restore an archive into an existing database, using a specific username
- `pg_restore -h host -p port -d db_name archive_file.dump` : restore an archive into an existing database, using a specific host and port
- `pg_restore -l archive_file.dump` : list database objects included in the archive
- `pg_restore -c -d db_name archive_file.dump` : clean database objects before creating them
- `pg_restore -j 2 -d db_name archive_file.dump` : use multiple jobs to do the restoring

## Categories
databases, archives, network

## Links
- More information: https://www.postgresql.org/docs/current/app-pgrestore.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_restore
