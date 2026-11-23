# innotop

MySQL and InnoDB transaction/status monitor.

## Usage
- `To monitor servers normally:` : the general form, as the manual gives it
- `innotop To monitor InnoDB status information from a file:` : another form of the same command
- `innotop /var/log/mysql/mysqld.err To run innotop non-interactively in a pipe-and-filter configuration:` : another form of the same command
- `innotop --count 5 -d 1 -n To monitor a database on another system using a particular username and password:` : another form of the same command
- `innotop -u <username> -p <password> -h <hostname>` : another form of the same command
- `innotop --color` : enable or disable terminal coloring
- `innotop --config` : specifies a configuration file to read

## Categories
monitoring, databases

## Links
- Documentation: https://metacpan.org/pod/DBD::MariaDB
