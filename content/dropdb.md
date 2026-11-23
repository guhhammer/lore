# dropdb

Remove a PostgreSQL database. A simple wrapper around the SQL command `DROP DATABASE`.

## Usage
- `dropdb database_name` : destroy a database
- `dropdb -i database_name` : request a verification prompt before any destructive actions
- `dropdb -U username database_name` : connect with a specific username and destroy a database
- `dropdb -W database_name` : force a password prompt before connecting to the database
- `dropdb -w database_name` : suppress a password prompt before connecting to the database
- `dropdb -h host database_name` : specify the server host name
- `dropdb -p port database_name` : specify the server port
- `dropdb -f database_name` : attempt to terminate existing connections before destroying the database

## Categories
databases, network, shell

## Links
- More information: https://www.postgresql.org/docs/current/app-dropdb.html
- tldr page: https://tldr.inbrowser.app/pages/common/dropdb
