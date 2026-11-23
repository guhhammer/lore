# doctl databases user

View details for, and create, database users.

## Usage
- `doctl d u command -t access_token` : run a `doctl databases user` command with an access token
- `doctl d u g database_id user_name` : retrieve details about a database user
- `doctl d u ls database_id` : retrieve a list of database users for a given database
- `doctl d u rs database_id user_name` : reset the auth password for a given user
- `doctl d u rs database_id user_name caching_sha2_password|mysql_native_password` : reset the MySQL auth plugn for a given user
- `doctl d u c database_id user_name` : create a user in the given database with a given username
- `doctl d u rm database_id user_name` : delete a user from the given database with the given username

## Categories
users, databases

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/databases/user/
- tldr page: https://tldr.inbrowser.app/pages/common/doctl-databases-user
