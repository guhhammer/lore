# doctl databases firewalls

Manage firewalls for database clusters.

## Usage
- `doctl d fw command -t access_token` : run a `doctl databases firewalls` command with an access token
- `doctl d fw ls` : retrieve a list of firewall rules for a given database
- `doctl d fw a database_id --rule droplet|k8s|ip_addr|tag|app:value` : add a database firewall rule to a given database
- `doctl d fw rm database_id rule_uuid` : remove a firewall rule for a given database

## Categories
databases, security

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/databases/firewalls/
- tldr page: https://tldr.inbrowser.app/pages/common/doctl-databases-firewalls
