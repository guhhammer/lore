# mysql

mysql is the command line client of MySQL and MariaDB. This page covers connecting, making users and databases, backing up and the queries people look up most.

## Tasks
### Connect
- `mysql -u USER -p` : log in as a user, asking for the password
- `mysql -u USER -p -h HOST -P 3306 DB` : connect to a remote server and open a database
- `sudo mysql` : log in as the admin through the local socket (Debian, Ubuntu)
- `mysql -u USER -p DB -e "SELECT NOW();"` : run one statement and leave
- `mysql -u USER -p DB < FILE.sql` : run a file of statements
### Databases and users
- `SHOW DATABASES;` : list databases
- `CREATE DATABASE DB CHARACTER SET utf8mb4;` : make a database
- `USE DB;` : switch to a database
- `CREATE USER 'NAME'@'localhost' IDENTIFIED BY 'PASSWORD';` : make a user
- `GRANT ALL PRIVILEGES ON DB.* TO 'NAME'@'localhost';` : let the user do everything in one database
- `SHOW GRANTS FOR 'NAME'@'localhost';` : see a user's rights
- `DROP USER 'NAME'@'localhost';` : delete a user
- `DROP DATABASE DB;` : delete a database and all its data
### Look around
- `SHOW TABLES;` : list the tables
- `DESCRIBE TABLE_NAME;` : columns and types of a table
- `SHOW CREATE TABLE TABLE_NAME;` : the statement that builds it
- `SELECT COUNT(*) FROM TABLE_NAME;` : how many rows
- `SHOW PROCESSLIST;` : running queries
### Back up and restore
- `mysqldump -u USER -p DB > backup.sql` : dump one database
- `mysqldump -u USER -p --all-databases > all.sql` : dump everything
- `mysqldump -u USER -p --single-transaction DB > backup.sql` : a consistent dump of InnoDB without locking
- `mysql -u USER -p DB < backup.sql` : restore a dump into an existing database
- `mysqldump -u USER -p DB | gzip > backup.sql.gz` : a compressed dump
- `gunzip < backup.sql.gz | mysql -u USER -p DB` : restore a compressed dump
### Queries
- `SELECT * FROM TABLE_NAME LIMIT 10;` : the first rows
- `ALTER USER 'NAME'@'localhost' IDENTIFIED BY 'NEW';` : change a password
- `FLUSH PRIVILEGES;` : reload the grant tables
- `quit` : leave the client

## Notes
- Statements typed inside the client end with a semicolon.
- `-pPASSWORD` on the command line shows the password in the process list: use bare `-p` and type it.
- `'user'@'localhost'` and `'user'@'%'` are different users.
- `DROP DATABASE` has no undo: take a dump first.
- Commands start with `mariadb` on newer MariaDB; `mysql` still works.

## See also
- postgresql
- sqlite
- mysqldump
- redis

## Links
- Manual: https://dev.mysql.com/doc/refman/8.4/en/mysql.html
- MariaDB: https://mariadb.com/kb/en/mysql-command-line-client/

## Categories
database
