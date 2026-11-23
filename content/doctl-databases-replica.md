# doctl databases replica

Manage read-only replicas associated with a database cluster.

## Usage
- `doctl d r command -t access_token` : run a `doctl databases replica` command with an access token
- `doctl d r g database_id replica_name` : retrieve information about a read-only database replica
- `doctl d r ls database_id` : retrieve list of read-only database replicas
- `doctl d r c database_id replica_name` : create a read-only database replica
- `doctl d r rm database_id replica_name` : delete a read-only database replica

## Categories
databases, containers

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/databases/replica/
- tldr page: https://tldr.inbrowser.app/pages/common/doctl-databases-replica
