# mariadb

The mariadb client tool.

## Usage
- `mariadb` : connect to MariaDB as the current user
- `mariadb db_name` : connect to a specific MariaDB database
- `mariadb -u user_name -p your_password db_name` : connect to a specific MariaDB database using username and password
- `mariadb --show-warning` : show warnings after every statement in interactive and batch mode
- `mariadb -s|-ss|-sss|--silent` : display less verbose outputs (can be used multiple times to produce less output)
- `mariadb <path/to/script.sql db_name >path/to/output.tab` : execute SQL statements from a script file
- `mariadb --debug-check` : check memory and open file usage at exit
- `mariadb -S path/to/socket_name` : connect using a socket file for local connections

## Categories
files, databases, development

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/mariadb
- Wikipedia: https://en.wikipedia.org/wiki/MariaDB
