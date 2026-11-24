# mongodb

MongoDB stores JSON-like documents. mongosh is its shell; mongodump and mongorestore make and load backups.

## Tasks
### Connect
- `mongosh` : connect to the local server
- `mongosh "mongodb://HOST:27017/DB"` : connect to a server and database
- `mongosh "mongodb://USER@HOST:27017/DB?authSource=admin"` : log in as a user, asking for the password
- `mongosh --eval "db.stats()"` : run one command and leave
### Look around (inside mongosh)
- `show dbs` : list databases
- `use DB` : switch to a database
- `show collections` : list collections
- `db.COLLECTION.countDocuments()` : how many documents
- `db.COLLECTION.getIndexes()` : list indexes
- `exit` : leave
### Documents
- `db.COLLECTION.insertOne({ name: "Ana", age: 30 })` : add a document
- `db.COLLECTION.find()` : read documents
- `db.COLLECTION.find({ age: { $gt: 25 } })` : filter them
- `db.COLLECTION.find({}, { name: 1, _id: 0 })` : choose which fields come back
- `db.COLLECTION.updateOne({ name: "Ana" }, { $set: { age: 31 } })` : change a document
- `db.COLLECTION.deleteOne({ name: "Ana" })` : delete a document
- `db.COLLECTION.createIndex({ name: 1 })` : make an index
- `db.COLLECTION.drop()` : delete a collection and its data
### Users
- `db.createUser({ user: "NAME", pwd: passwordPrompt(), roles: ["readWrite"] })` : make a user for the current database
- `db.getUsers()` : list users of this database
- `db.changeUserPassword("NAME", passwordPrompt())` : change a password
### Back up and restore
- `mongodump --uri "mongodb://HOST:27017/DB" --out DIR` : dump a database to a folder
- `mongodump --uri "mongodb://HOST:27017/DB" --archive=backup.gz --gzip` : dump to one compressed file
- `mongorestore --uri "mongodb://HOST:27017" DIR` : restore from a folder
- `mongorestore --uri "mongodb://HOST:27017" --archive=backup.gz --gzip` : restore from a compressed file
- `mongoexport --uri "mongodb://HOST:27017/DB" -c COLLECTION --out FILE.json` : export a collection as JSON

## Notes
- Databases and collections appear when you first write to them: `use DB` alone creates nothing.
- Turn on authentication before exposing the server, and bind it to localhost or a private network.
- mongodump keeps BSON and indexes; mongoexport gives JSON that loses some types.
- `mongosh` replaced the old `mongo` shell.
- The tools mongodump and mongorestore come in the separate MongoDB Database Tools package.

## See also
- redis
- postgresql
- mysql

## Links
- mongosh: https://www.mongodb.com/docs/mongodb-shell/
- Database Tools: https://www.mongodb.com/docs/database-tools/

## Categories
database
