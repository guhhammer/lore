# pg_dump

Extract a PostgreSQL database into a script file or other archive file.

## Usage
- `pg_dump db_name >output_file.sql` : dump database into an SQL-script file
- `pg_dump -U username db_name >output_file.sql` : dump a database into an SQL-script file, using a specific username
- `pg_dump -h host -p port db_name >output_file.sql` : dump a database into an SQL-script file, using a specific host and port
- `pg_dump -F c db_name >output_file.dump` : dump a database into a custom-format archive file
- `pg_dump -a db_name >path/to/output_file.sql` : dump only database data into an SQL-script file
- `pg_dump -s db_name >path/to/output_file.sql` : dump only schema (data definitions) into an SQL-script file

## Categories
databases, files, development

## Links
- More information: https://www.postgresql.org/docs/current/app-pgdump.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_dump
