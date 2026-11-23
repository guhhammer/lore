# pg_dumpall

Extract a PostgreSQL database cluster into a script file or other archive file.

## Usage
- `pg_dumpall >path/to/file.sql` : dump all databases
- `pg_dumpall -U username >path/to/file.sql` : dump all databases using a specific username
- `pg_dumpall -h host -p port >output_file.sql` : dump all databases using a specific host and port
- `pg_dumpall -a >path/to/file.sql` : dump only database data into an SQL-script file
- `pg_dumpall -s >output_file.sql` : dump only schema (data definitions) into an SQL-script file

## Categories
files, databases, archives

## Links
- More information: https://www.postgresql.org/docs/current/app-pg-dumpall.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_dumpall
