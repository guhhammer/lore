# pg_createsubscriber

Convert a physical replica into a new logical replica.

## Usage
- `pg_createsubscriber -d dbname -D path/to/data -P connstr` : convert a physical replica to a logical replica for a specific database
- `pg_createsubscriber -n -d dbname -D path/to/data -P connstr` : simulate converting a physical replica to a logical replica without modifying the target directory
- `pg_createsubscriber -T -d dbname -D path/to/data -P connstr` : enable two-phase commit for the subscription
- `pg_createsubscriber -v -d dbname -D path/to/data -P connstr` : convert with verbose output
- `pg_createsubscriber -?` : display help

## Categories
files

## Links
- More information: https://www.postgresql.org/docs/current/app-pgcreatesubscriber.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_createsubscriber
