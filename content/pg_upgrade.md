# pg_upgrade

Upgrade a PostgreSQL database cluster to a new major version.

## Usage
- `pg_upgrade -c -b path/to/old_bin -B path/to/new_bin -d path/to/old_data -D path/to/new_data` : check clusters before upgrading
- `pg_upgrade -b path/to/old_bin -B path/to/new_bin -d path/to/old_data -D path/to/new_data` : perform the actual upgrade
- `pg_upgrade -j njobs -b path/to/old_bin -B path/to/new_bin -d path/to/old_data -D path/to/new_data` : use multiple parallel jobs during the upgrade
- `pg_upgrade -p port -P port -b path/to/old_bin -B path/to/new_bin -d path/to/old_data -D path/to/new_data` : specify the old and new PostgreSQL ports
- `pg_upgrade -k -b path/to/old_bin -B path/to/new_bin -d path/to/old_data -D path/to/new_data` : use hard links instead of copying files to the new cluster
- `pg_upgrade -?` : display help

## Categories
files, packages, containers

## Links
- More information: https://www.postgresql.org/docs/current/pgupgrade.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_upgrade
