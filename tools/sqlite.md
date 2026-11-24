# sqlite

SQLite keeps a whole database in one file and needs no server. The sqlite3 command opens it for queries, dumps and imports.

## Tasks
### Open
- `sqlite3 FILE.db` : open or create a database file
- `sqlite3 FILE.db "SELECT * FROM TABLE_NAME;"` : run one statement and leave
- `sqlite3 -readonly FILE.db` : open it without allowing writes
- `sqlite3 -header -column FILE.db` : readable columns with a header
- `sqlite3 :memory:` : a temporary database in memory
### Look around (inside sqlite3)
- `.tables` : list tables
- `.schema` : show every CREATE statement
- `.schema TABLE_NAME` : one table's definition
- `.headers on` : show column names
- `.mode column` : align the output (also csv, json, table)
- `.help` : list the dot commands
- `.quit` : leave
### Import and export
- `.import --csv FILE.csv TABLE_NAME` : load a CSV into a table
- `sqlite3 -csv -header FILE.db "SELECT * FROM TABLE_NAME;" > out.csv` : export a query as CSV
- `.output FILE.txt` : send results to a file until `.output stdout`
- `.read FILE.sql` : run a file of statements
### Back up
- `sqlite3 FILE.db ".backup backup.db"` : make a safe copy while the database is in use
- `sqlite3 FILE.db .dump > dump.sql` : dump as SQL text
- `sqlite3 NEW.db < dump.sql` : rebuild a database from a dump
- `sqlite3 FILE.db "VACUUM;"` : shrink the file after deletes
- `sqlite3 FILE.db "PRAGMA integrity_check;"` : check the file is sound
### Queries
- `CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);` : make a table
- `INSERT INTO users (name) VALUES ('Ana');` : add a row
- `SELECT * FROM users WHERE name LIKE 'A%';` : filter rows
- `PRAGMA table_info(users);` : columns of a table
- `PRAGMA foreign_keys = ON;` : turn on foreign keys (per connection)

## Notes
- Copying the `.db` file while a program writes to it can give a broken copy: use `.backup`.
- Statements end with a semicolon; dot commands do not.
- Foreign keys are off by default and must be enabled on each connection.
- Install the command with the package `sqlite3`.

## See also
- mysql
- postgresql
- duckdb

## Links
- CLI: https://www.sqlite.org/cli.html
- SQL: https://www.sqlite.org/lang.html

## Categories
database
