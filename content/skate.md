# skate

Simple and powerful key-value store.

## Usage
- `skate set "key" "value"` : store a key and a value on the default database
- `skate list` : show your keys saved on the default database
- `skate delete "key"` : delete key and value from the default database
- `skate set "key"@"database_name" "value"` : create a new key and value in a new database
- `skate list @"database_name"` : show your keys saved in a non default database
- `skate delete "key"@"database_name"` : delete key and value from a specific database
- `skate list-dbs` : show the databases available
- `skate reset @"database_name"` : delete local db and pull down fresh copy from Charm Cloud

## Categories
databases

## Links
- More information: https://manned.org/skate
- tldr page: https://tldr.inbrowser.app/pages/common/skate
