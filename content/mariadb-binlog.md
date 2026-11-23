# mariadb-binlog

Utility for processing MariaDB binary log files.

## Usage
- `mariadb-binlog path/to/binlog` : show events from a specific binary log file
- `mariadb-binlog -d database_name path/to/binlog` : show entries from a binary log for a specific database
- `mariadb-binlog --start-datetime '2022-01-01 01:00:00' --stop-datetime '2022-02-01 01:00:00' path/to/binlog` : show events from a binary log between specific dates
- `mariadb-binlog -j 100 --stop-position 200 path/to/binlog` : show events from a binary log between specific positions
- `mariadb-binlog -h hostname path/to/binlog` : show binary log from a MariaDB server on the given host

## Categories
monitoring, files, network

## Links
- More information: https://manned.org/mariadb-binlog
- tldr page: https://tldr.inbrowser.app/pages/common/mariadb-binlog
