# postgresql

PostgreSQL is a database server; psql is its command line client. This page covers connecting, users, backup and restore and the commands for looking around.

## Tasks
### Connect
- `sudo -u postgres psql` : open psql as the admin user on the same machine
- `psql -U USER -h HOST -p 5432 -d DB` : connect to a server
- `psql "postgresql://USER@HOST:5432/DB"` : connect with a URL
- `psql -d DB -c "SELECT now();"` : run one statement and leave
- `psql -d DB -f FILE.sql` : run a file of statements
### Look around (inside psql)
- `\l` : list databases
- `\c DB` : connect to another database
- `\dt` : list tables
- `\d TABLE_NAME` : describe a table
- `\du` : list roles
- `\x` : switch to one-column-per-row output
- `\timing` : show how long each query took
- `\q` : quit
### Users and databases
- `createuser --interactive NAME` : make a role by answering questions
- `createdb -O USER DB` : make a database owned by a user
- `ALTER USER NAME WITH PASSWORD 'PASSWORD';` : set a password
- `GRANT ALL PRIVILEGES ON DATABASE DB TO NAME;` : give a role rights on a database
- `dropdb DB` : delete a database and its data
- `dropuser NAME` : delete a role
### Back up and restore
- `pg_dump -U USER DB > backup.sql` : dump one database as SQL
- `pg_dump -U USER -Fc DB > backup.dump` : dump in the compressed custom format
- `pg_restore -U USER -d DB backup.dump` : restore a custom-format dump
- `psql -U USER -d DB < backup.sql` : restore a SQL dump
- `pg_dumpall -U postgres > all.sql` : dump every database and role
### Check the server
- `SELECT version();` : the server version
- `SELECT pg_size_pretty(pg_database_size('DB'));` : the size of a database
- `SELECT * FROM pg_stat_activity;` : who is connected and what they run
- `SELECT pg_terminate_backend(PID);` : end one connection

## Notes
- On Debian and Ubuntu the admin is the system user `postgres`: use `sudo -u postgres`.
- `pg_restore` only reads the custom format; plain SQL dumps go through `psql`.
- Put the password in `~/.pgpass` (mode 600) instead of typing it into scripts.
- `\` commands belong to psql, not SQL, and need no semicolon.
- `dropdb` and `DROP` statements delete data for good.

## See also
- mysql
- sqlite
- pg_dump
- redis

## Links
- psql: https://www.postgresql.org/docs/current/app-psql.html
- Backup: https://www.postgresql.org/docs/current/backup.html

## Categories
database
