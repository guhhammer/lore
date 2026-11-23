# rsql

SQL client to interface with databases and other data sources inside the terminal.

## Usage
- `rsql` : enter interactive mode
- `rsql --url "postgresql://user:pass@localhost/mydb"` : connect to a database (e.g. PostgreSQL)
- `rsql --url "postgresql://user:pass@localhost/db?sslmode=require"` : connect to a PostgreSQL database with SSL
- `rsql --url "mysql://user:pass@localhost/db?charset=utf8mb4"` : connect to a MySQL database with a specified charset
- `rsql --url "sqlite://database.db" -- "SELECT * FROM users LIMIT 10"` : run a query and exit
- `rsql --url "sqlite://db.sqlite" --format json` : set default format
- `rsql --url "delimited://data.txt?separator=|&has_header=true"` : connect to file and use custom line separator

## Categories
databases, network, users

## Links
- More information: https://github.com/theseus-rs/rsql
- tldr page: https://tldr.inbrowser.app/pages/common/rsql
