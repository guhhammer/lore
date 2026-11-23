# mysqldump

Back up MySQL databases.

## Usage
- `mysqldump --user user --password database_name --result-file=path/to/file.sql` : create a backup (user will be prompted for a password)
- `mysqldump --user user --password database_name table_name >path/to/file.sql` : back up a specific table redirecting the output to a file (user will be prompted for a password)
- `mysqldump --user user --password --all-databases >path/to/file.sql` : back up all databases redirecting the output to a file (user will be prompted for a password)
- `mysqldump --host=ip_or_hostname --user user --password --all-databases >path/to/file.sql` : back up all databases from a remote host, redirecting the output to a file (user will be prompted for a password)

## See also
- mysql

## Categories
users, files, databases

## Links
- More information: https://dev.mysql.com/doc/refman/en/mysqldump.html
- tldr page: https://tldr.inbrowser.app/pages/common/mysqldump
