# usql

Universal CLI interface for SQL databases.

## Usage
- `usql sqlserver|mysql|postgres|sqlite3|...://username:password@host:port/database_name` : connect to a specific database
- `usql -f path/to/query.sql` : execute commands from a file
- `usql -c "sql_command"` : execute a specific SQL command
- `command` : [Interactive] Run an SQL command in the `usql` prompt
- `\d` : [Interactive] Display the database schema
- `\g path/to/file_with_results` : [Interactive] Export query results to a specific file
- `\copy path/to/data.csv table_name` : [Interactive] Import data from a CSV file into a specific table

## Categories
databases, files, network

## Links
- More information: https://github.com/xo/usql#using
- tldr page: https://tldr.inbrowser.app/pages/common/usql
