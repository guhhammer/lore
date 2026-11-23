# csvsql

Generate SQL statements for a CSV file or execute those statements directly on a database. Included in csvkit.

## Usage
- `csvsql path/to/data.csv` : generate a `CREATE TABLE` SQL statement for a CSV file
- `csvsql --insert --db "mysql://user:password@host/database" data.csv` : import a CSV file into an SQL database
- `csvsql --query "select * from 'data'" data.csv` : run an SQL query on a CSV file

## Categories
databases, text, files

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/csvsql.html
- tldr page: https://tldr.inbrowser.app/pages/common/csvsql
