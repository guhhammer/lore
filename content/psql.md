# psql

PostgreSQL client.

## Usage
- `psql database` : connect to the database. By default, it connects to the local socket using port 5432 with the currently logged in user
- `psql -h host -p port -U username database` : connect to the database on given server host running on given port with given username, without a password prompt
- `psql -h host -p port -U username -W database` : connect to the database; user will be prompted for password
- `psql -c 'query' database` : execute a single SQL query or PostgreSQL command on the given database (useful in shell scripts)
- `psql database -f path/to/file.sql` : execute commands from a file on the given database

## Categories
databases

## Links
- More information: https://www.postgresql.org/docs/current/app-psql.html
- tldr page: https://tldr.inbrowser.app/pages/common/psql
