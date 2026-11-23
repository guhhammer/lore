# mongo

The legacy MongoDB shell. See `mongosh` for the new shell. Note: All connection options can be replaced with one string: `mongodb://user@host:port/db_name?authSource=authdb_name`.

## Usage
- `mongo` : connect to a local database on the default port (`mongodb://localhost:27017`)
- `mongo --host host --port port db_name` : connect to a database
- `mongo --host host --port port --username username --authenticationDatabase authdb_name db_name` : authenticate using the specified username on the specified database (you will be prompted for a password)
- `mongo --eval 'JSON.stringify(db.foo.findOne())' db_name` : evaluate a JavaScript expression on a database

## Categories
network, databases, shell

## Links
- More information: https://www.mongodb.com/docs/mongodb-shell/
- tldr page: https://tldr.inbrowser.app/pages/common/mongo
