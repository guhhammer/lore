# mysql

The MySQL tool.

## Usage
- `mysql database_name` : connect to a database
- `mysql -u user -p database_name` : connect to a database, user will be prompted for a password
- `mysql -h database_host database_name` : connect to a database on another host
- `mysql -S path/to/socket.sock` : connect to a database through a Unix socket
- `mysql -e "source filename.sql" database_name` : execute SQL statements in a script file (batch file)
- `mysql <path/to/backup.sql -u user -p database_name` : restore a database from a backup created with `mysqldump` (user will be prompted for a password)
- `mysql <path/to/backup.sql -u user -p` : restore all databases from a backup (user will be prompted for a password)

## Categories
databases

## Links
- More information: https://manned.org/mysql
- tldr page: https://tldr.inbrowser.app/pages/common/mysql
