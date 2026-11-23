# mongodump

Utility to export the contents of a MongoDB instance.

## Usage
- `mongodump` : create a dump of all databases (this will place the files inside a directory called "dump")
- `mongodump -o path/to/directory` : specify an output location for the dump
- `mongodump -d database_name` : create a dump of a given database
- `mongodump -c collection_name -d database_name` : create a dump of a given collection within a given database
- `mongodump -h host --port port` : connect to a given host running on a given port and create a dump
- `mongodump -u username database -p` : create a dump of a given database with a given username; user will be prompted for password
- `mongodump --uri connection_string` : create a dump from a specific instance; host, user, password, and database will be defined in the connection string

## Categories
network, databases, users

## Links
- More information: https://www.mongodb.com/docs/database-tools/mongodump/
- tldr page: https://tldr.inbrowser.app/pages/common/mongodump
