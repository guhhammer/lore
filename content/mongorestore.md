# mongorestore

Utility to import a collection or database from a binary dump into a MongoDB instance.

## Usage
- `mongorestore --db database_name path/to/directory` : import a BSON data dump from a directory to a MongoDB database
- `mongorestore --host database_host:port --db database_name --username username path/to/directory --password` : import a BSON data dump from a directory to a given database in a MongoDB server host, running at a given port, with user authentication (user will be prompted for password)
- `mongorestore --db database_name path/to/file` : import a collection from a BSON file to a MongoDB database
- `mongorestore --host database_host:port --db database_name --username username path/to/file --password` : import a collection from a BSON file to a given database in a MongoDB server host, running at a given port, with user authentication (user will be prompted for password)

## Categories
files, network, databases

## Links
- More information: https://www.mongodb.com/docs/database-tools/mongorestore/
- tldr page: https://tldr.inbrowser.app/pages/common/mongorestore
