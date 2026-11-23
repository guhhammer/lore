# pg_recvlogical

Control PostgreSQL logical decoding streams.

## Usage
- `pg_recvlogical -d dbname -S slot_name --create-slot` : create a new logical replication slot
- `pg_recvlogical -d dbname -S slot_name --start -f filename` : start streaming changes from a logical replication slot to a file
- `pg_recvlogical -d dbname -S slot_name --drop-slot` : drop a logical replication slot
- `pg_recvlogical -d dbname -S slot_name --create-slot -t` : create a slot with two-phase commit enabled
- `pg_recvlogical -?` : display help

## Links
- More information: https://www.postgresql.org/docs/current/app-pgrecvlogical.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_recvlogical
