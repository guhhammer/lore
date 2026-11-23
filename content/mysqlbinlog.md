# mysqlbinlog

Utility for processing MySQL binary log files.

## Usage
- `mysqlbinlog path/to/binlog` : show events from a specific binary log file
- `mysqlbinlog -d database_name path/to/binlog` : show entries from a binary log for a specific database
- `mysqlbinlog --start-datetime '2022-01-01 01:00:00' --stop-datetime '2022-02-01 01:00:00' path/to/binlog` : show events from a binary log between specific dates
- `mysqlbinlog -j 100 --stop-position 200 path/to/binlog` : show events from a binary log between specific positions
- `mysqlbinlog -h hostname path/to/binlog` : show binary log from a MySQL server on the given host

## Categories
monitoring, databases, files

## Links
- More information: https://dev.mysql.com/doc/refman/en/mysqlbinlog.html
- tldr page: https://tldr.inbrowser.app/pages/common/mysqlbinlog
