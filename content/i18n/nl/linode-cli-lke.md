# linode-cli lke

Beheer Linode Kubernetes Engine (LKE) clusters.

## Usage
- `linode-cli lke clusters list` : toon alle LKE-clusters
- `linode-cli lke clusters create --region regio --type type --node-type node_type --nodes-count aantal` : maak een nieuw LKE-cluster
- `linode-cli lke clusters view cluster_id` : toon details van een specifiek LKE-cluster
- `linode-cli lke clusters update cluster_id --node-type nieuwe_node_type` : update een bestaand LKE-cluster
- `linode-cli lke clusters delete cluster_id` : verwijder een LKE-cluster

## Links
- More information: https://techdocs.akamai.com/cloud-computing/docs/cli-commands-for-lke
