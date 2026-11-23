# sqlite-utils

Manipulate SQLite databases in a number of different ways.

## Usage
- `sqlite-utils create-database path/to/database.db` : create a database
- `sqlite-utils create-table path/to/database.db table_name id integer name text height float photo blob --pk id` : create a table
- `sqlite-utils tables path/to/database.db` : list tables
- `echo '[ {"id": 1, "name": "Linus Torvalds"}, {"id": 2, "name": "Steve Wozniak"}, {"id": 3, "name": "Tony Hoare"} ]' | sqlite-utils upsert path/to/database.db table_name - --pk id` : upsert a record
- `sqlite-utils rows path/to/database.db table_name` : select records
- `sqlite-utils query path/to/database.db "delete from table_name where name = 'Tony Hoare'"` : delete a record
- `sqlite-utils drop-table path/to/database.db table_name` : drop a table
- `sqlite-utils -h` : display help

## Categories
databases, files, media

## Links
- More information: https://sqlite-utils.datasette.io/en/stable/cli.html
- tldr page: https://tldr.inbrowser.app/pages/common/sqlite-utils
