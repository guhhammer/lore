# doctl kubernetes cluster

Manage Kubernetes clusters and view configuration options relating to clusters.

## Usage
- `doctl k c c --count 3 --region nyc1 --size s-1vcpu-2gb --version latest cluster_name` : create a Kubernetes cluster
- `doctl k c ls` : list all Kubernetes clusters
- `doctl k c cfg s cluster_name` : fetch and save the kubeconfig
- `doctl k c gu cluster_name` : check for available upgrades
- `doctl k c upgrade cluster_name` : upgrade a cluster to a new Kubernetes version
- `doctl k c d cluster_name` : delete a cluster

## Categories
containers

## Links
- More information: https://docs.digitalocean.com/reference/doctl/reference/kubernetes/cluster/
- tldr page: https://tldr.inbrowser.app/pages/common/doctl-kubernetes-cluster
