# postgres

Run the PostgreSQL database server.

## Usage
- `postgres -p 5433` : start the server on a specific port (defaults to 5432)
- `postgres -c shared_buffers=128MB` : set a runtime parameter (short form)
- `postgres --shared-buffers=128MB` : set a runtime parameter (long form)
- `postgres --single -D path/to/datadir my_database` : start in single-user mode for a specific database (defaults to the user name)

## Categories
databases, network

## Links
- More information: https://www.postgresql.org/docs/current/app-postgres.html
- tldr page: https://tldr.inbrowser.app/pages/common/postgres
