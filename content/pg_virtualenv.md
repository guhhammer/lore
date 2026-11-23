# pg_virtualenv

Create a throw-away PostgreSQL environment for running regression tests.

## Usage
- `pg_virtualenv [OPTIONS] [-v 'version ...'] [command]` : the general form, as the manual gives it
- `pg_virtualenv -a` : use all PostgreSQL server versions installed
- `pg_virtualenv -s` : launch a shell inside the virtual environment when command fails
- `pg_virtualenv -t` : install clusters in a temporary directory, even when running as root
- `pg_virtualenv -h` : show program help

## Categories
development
