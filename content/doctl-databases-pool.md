# doctl databases pool

Manage connection pools for your database cluster.

## Usage
- `doctl d p command -t access_token` : run a `doctl databases pool` command with an access token
- `doctl d p g database_id pool_name` : retrieve information about a database connection pool
- `doctl d p ls database_id` : list connection pools for a database cluster
- `doctl d p c database_id pool_name --db new_pool_name --size pool_size` : create a connection pool for a database
- `doctl d p rm database_id pool_name` : delete a connection pool for a database

## Categories
databases, containers

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/databases/pool/
- tldr page: https://tldr.inbrowser.app/pages/common/doctl-databases-pool
