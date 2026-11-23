# vacuumdb

Garbage-collect and analyze a PostgreSQL database.

## Usage
- `vacuumdb database_name` : vacuum a specific database
- `vacuumdb -a` : vacuum all databases
- `vacuumdb -t table_name database_name` : vacuum a specific table in a database
- `vacuumdb -z database_name` : vacuum and update statistics for the query planner
- `vacuumdb -f database_name` : perform a full vacuum (more aggressive, locks tables, rewrites the whole table)
- `vacuumdb -v database_name` : vacuum with verbose output
- `vacuumdb --jobs number_of_jobs database_name` : vacuum a database using multiple parallel jobs

## Categories
databases

## Links
- More information: https://www.postgresql.org/docs/current/app-vacuumdb.html
- tldr page: https://tldr.inbrowser.app/pages/common/vacuumdb
