# pg_checksums

Enable, disable, or check data checksums in a PostgreSQL database cluster.

## Usage
- `pg_checksums -D path/to/data` : check data checksums in a database cluster
- `pg_checksums -e -D path/to/data` : enable data checksums in a database cluster
- `pg_checksums -d -D path/to/data` : disable data checksums in a database cluster
- `pg_checksums -v -D path/to/data` : check data checksums with verbose output
- `pg_checksums -P -D path/to/data` : check data checksums showing progress
- `pg_checksums -?` : display help

## Categories
containers, databases, files

## Links
- More information: https://www.postgresql.org/docs/current/app-pgchecksums.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_checksums
