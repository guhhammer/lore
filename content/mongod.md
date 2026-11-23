# mongod

The MongoDB database server.

## Usage
- `mongod --dbpath path/to/directory` : specify the storage directory (default: `/data/db` on Linux and macOS, `C:\data\db` on Windows)
- `mongod --config path/to/file` : specify a configuration file
- `mongod --port port` : specify the port to listen on (default: 27017)
- `mongod --profile 0|1|2` : specify the database profiling level. 0 is off, 1 is only slow operations, 2 is all (default: 0)

## Categories
files, network, databases

## Links
- More information: https://www.mongodb.com/docs/manual/reference/program/mongod/
- tldr page: https://tldr.inbrowser.app/pages/common/mongod
