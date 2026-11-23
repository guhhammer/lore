# pg_amcheck

Check for corruption in one or more PostgreSQL databases.

## Usage
- `pg_amcheck dbname` : check a specific database
- `pg_amcheck -a` : check all databases
- `pg_amcheck -d pattern` : check databases matching the specified pattern
- `pg_amcheck -t pattern dbname` : check specific tables within a database
- `pg_amcheck -s pattern dbname` : check specific schemas within a database
- `pg_amcheck -?` : display help

## Categories
text, databases

## Links
- More information: https://www.postgresql.org/docs/current/app-pgamcheck.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_amcheck
