# reindexdb

Rebuild indexes in a PostgreSQL database.

## Usage
- `reindexdb database_name` : reindex a specific database
- `reindexdb database_name -h hostname -p port -U username` : reindex a specific database using connection options
- `reindexdb -a` : reindex all databases
- `reindexdb database_name -t table_name` : reindex a specific table in a database
- `reindexdb database_name -i index_name` : reindex a specific index in a database
- `reindexdb database_name -S schema_name` : reindex a specific schema in a database
- `reindexdb database_name -v` : reindex with verbose output
- `reindexdb database_name -j number_of_jobs` : reindex a database using multiple parallel jobs

## Categories
databases, network

## Links
- More information: https://www.postgresql.org/docs/current/app-reindexdb.html
- tldr page: https://tldr.inbrowser.app/pages/common/reindexdb
