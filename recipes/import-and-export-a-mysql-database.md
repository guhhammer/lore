# import and export a mysql database

Load a .sql file into MySQL or MariaDB, dump a database to one, and connect from the terminal.

## Steps
- `mysql -u NAME -p` : connect; it asks for the password
- `mysql -u NAME -p dbname < dump.sql` : load a .sql file into a database
- `mysqldump -u NAME -p dbname > dump.sql` : dump a database to a file
- `mysqldump -u NAME -p --all-databases > all.sql` : every database
- `gunzip < dump.sql.gz | mysql -u NAME -p dbname` : load a compressed dump
- `mysql -u NAME -p -e 'SHOW DATABASES;'` : run one query and quit

## Notes
- -h host connects to another machine; -P to another port.

## See also
- mysql
- mysqldump

## Categories
databases
