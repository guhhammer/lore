# sequelize

Promise-based Node.js ORM for Postgres, MySQL, MariaDB, SQLite, and Microsoft SQL Server.

## Usage
- `sequelize model:generate --name table_name --attributes field1:integer,field2:string,field3:boolean` : create a model with 3 fields and a migration file
- `sequelize db:migrate` : run the migration file
- `sequelize db:migrate:undo:all` : revert all migrations
- `sequelize seed:generate --name seed_filename` : create a seed file with the specified name to populate the database
- `sequelize db:seed:all` : populate database using all seed files

## Categories
databases, files

## Links
- More information: https://github.com/sequelize/cli/blob/main/README.md
- tldr page: https://tldr.inbrowser.app/pages/common/sequelize
