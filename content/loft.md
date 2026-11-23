# loft

Install and manage multi-tenant Kubernetes environments using virtual clusters.

## Usage
- `loft start` : install or upgrade Loft in the current Kubernetes cluster
- `loft login https://loft.example.com` : authenticate to a remote Loft instance
- `loft create vcluster vcluster_name -s space_name -c cluster_name` : create a virtual cluster with a specific space and cluster
- `loft list vclusters` : list all virtual clusters
- `loft use vcluster vcluster_name` : switch context to a specific virtual cluster
- `loft delete vcluster vcluster_name` : delete a virtual cluster
- `loft vars username` : show the current Loft username
- `loft uninstall` : uninstall Loft from the cluster

## Categories
containers, packages, network

## Links
- More information: https://loft.sh/docs/cli/loft/
- tldr page: https://tldr.inbrowser.app/pages/common/loft
