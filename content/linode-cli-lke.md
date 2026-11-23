# linode-cli lke

Manage Linode Kubernetes Engine (LKE) clusters.

## Usage
- `linode-cli lke clusters list` : list all LKE clusters
- `linode-cli lke clusters create --region region --type type --node-type node_type --nodes-count count` : create a new LKE cluster
- `linode-cli lke clusters view cluster_id` : view details of a specific LKE cluster
- `linode-cli lke clusters update cluster_id --node-type new_node_type` : update an existing LKE cluster
- `linode-cli lke clusters delete cluster_id` : delete an LKE cluster

## Categories
containers

## Links
- More information: https://techdocs.akamai.com/cloud-computing/docs/cli-commands-for-lke
- tldr page: https://tldr.inbrowser.app/pages/common/linode-cli-lke
