# sqlx

Utility for SQLx, the Rust SQL toolkit.

## Usage
- `sqlx database create` : create the database specified in the `$DATABASE_URL` environment variable
- `sqlx database drop -D database_url` : drop the specified database
- `sqlx migrate add -r migration_description` : create a new pair of up and down migration files with the given description in the "migrations" directory
- `sqlx migrate run -D database_url` : run all pending migrations for the specified database
- `sqlx migrate revert -D database_url` : revert the latest migration for the specified database

## Categories
databases

## Links
- More information: https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md
- tldr page: https://tldr.inbrowser.app/pages/common/sqlx
