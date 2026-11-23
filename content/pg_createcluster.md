# pg_createcluster

Create a new PostgreSQL cluster.

## Usage
- `pg_createcluster [options] version name [-- initdb options]` : the general form, as the manual gives it
- `pg_createcluster -g group` : change the group of the cluster related data files
- `pg_createcluster -l path` : explicitly set the path for the postgres(1) server log file
- `pg_createcluster --locale=locale` : set the default locale for the database cluster
- `pg_createcluster --lc-time=locale` : like --locale, but only sets the locale in the specified category

## Categories
system, containers, files
