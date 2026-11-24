# back up and restore postgresql

Run a .sql file, dump a PostgreSQL database and load it back, from the terminal.

## Steps
- `sudo -u postgres psql` : connect as the postgres admin ("role does not exist": use this)
- `psql -U NAME -d dbname -f file.sql` : run a .sql file
- `pg_dump -U NAME -d dbname -F c -f db.dump` : dump a database in pg_restore's format
- `pg_restore -U NAME -d dbname db.dump` : load it back
- `pg_dump -U NAME dbname > dump.sql` : a plain .sql dump, loaded with psql -f
- `psql -U NAME -c 'SELECT version();'` : which PostgreSQL this is

## Notes
- A password without typing it: a ~/.pgpass file, or PGPASSWORD for one command.
- `sudo -u postgres createuser --interactive` makes a role for your user.

## See also
- psql
- pg_dump
- pg_restore

## Categories
databases
