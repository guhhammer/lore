# typeorm

A JavaScript ORM that can run on Node.js, browser, Cordova, Ionic, React Native, NativeScript, and Electron platforms.

## Usage
- `typeorm init` : generate a new initial TypeORM project structure
- `typeorm migration:create --name migration_name` : create an empty migration file
- `typeorm migration:generate --name migration_name` : create a migration file with the SQL statements to update the schema
- `typeorm migration:run` : run all pending migrations
- `typeorm entity:create --name entity --dir path/to/directory` : create a new entity file in a specific directory
- `typeorm schema:log` : display the SQL statements to be executed by `typeorm schema:sync` on the default connection
- `typeorm query sql_sentence` : execute a specific SQL statement on the default connection
- `typeorm subcommand --help` : display help for a subcommand

## Categories
files, databases

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/typeorm
