# doctl databases db

Manage databases that are served by a database cluster.

## Usage
- `doctl d db command -t access_token` : run a `doctl databases db` command with an access token
- `doctl d db g database_id database_name` : retrieve the name of the given database hosted in the given database cluster
- `doctl d db ls database_id` : list existing databases hosted within a given database cluster
- `doctl d db c database_id database_name` : create a database with the given name in the given database cluster
- `doctl d db rm database_id database_name` : delete the database with the given name in the given database cluster

## Categories
databases, containers

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/databases/db/
- tldr page: https://tldr.inbrowser.app/pages/common/doctl-databases-db
