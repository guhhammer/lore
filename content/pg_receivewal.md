# pg_receivewal

Stream the write-ahead log from a running PostgreSQL cluster.

## Usage
- `pg_receivewal -D path/to/directory` : stream WAL to a local directory (minimum required)
- `pg_receivewal -v -h host -p port -U username -D path/to/directory` : stream WAL to a local directory, specifying host, port, and username, with verbose output
- `pg_receivewal -S slot_name --create-slot -h host -p port -U username -D path/to/directory` : use replication slot (create-if-needed)
- `pg_receivewal -E lsn -D path/to/directory -h host -p port -U username` : stop at a given WAL position (LSN)
- `pg_receivewal -n -D path/to/directory -h host -p port -U username` : control looping on failure
- `pg_receivewal --synchronous -D path/to/directory -h host -p port -U username` : flush data synchronously (force WAL writes immediately)
- `pg_receivewal -Z level|method -D path/to/directory -h host -p port -U username` : compress WAL output (`gzip`, level 0-9)
- `pg_receivewal -s seconds -D path/to/directory -h host -p port -U username` : set status reporting interval

## Categories
files, network, media

## Links
- More information: https://www.postgresql.org/docs/current/app-pgreceivewal.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_receivewal
