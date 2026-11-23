# mysqlsh

Advanced client for MySQL, supporting SQL, JavaScript, and Python. It offers features for managing InnoDB clusters and document store collections.

## Usage
- `mysqlsh` : start MySQL Shell in interactive mode
- `mysqlsh --user username --host hostname --port port` : connect to a MySQL server
- `mysqlsh --user username --execute 'sql_statement'` : execute an SQL statement on the server and exit
- `mysqlsh --js` : start MySQL Shell in JavaScript mode
- `mysqlsh --py` : start MySQL Shell in Python mode
- `mysqlsh --import path/to/file.json --schema schema_name --collection collection_name` : import JSON documents into a MySQL collection
- `mysqlsh --verbose` : enable verbose output

## Categories
databases, network, documents

## Links
- More information: https://manned.org/mysqlsh
- tldr page: https://tldr.inbrowser.app/pages/common/mysqlsh
