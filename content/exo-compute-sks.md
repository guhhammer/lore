# exo compute sks

Manage the Exoscale Scalable Kubernetes Service (SKS).

## Usage
- `exo compute sks versions` : list the supported SKS cluster versions
- `exo compute sks create cluster_name -z zone` : create a new SKS cluster
- `exo compute sks list` : list all SKS clusters
- `exo compute sks kubeconfig cluster_name|id user --ttl 1800 -z zone` : generate a Kubernetes kubeconfig file for an SKS cluster that expires in 1800 seconds
- `exo compute sks nodepool add cluster_name|id nodepool_name --size 3 -z zone` : create and add a Nodepool containing 3 nodes to an SKS cluster
- `exo compute sks nodepool delete cluster_name|id nodepool_name|id` : remove a Nodepool from an SKS cluster
- `exo compute sks nodepool evict cluster_name|id nodepool_name|id node_name|id` : evict a Node from a Nodepool in an SKS cluster
- `exo compute sks update cluster_name|id --enable-csi-addon -z zone` : enable the Exoscale CSI driver for an existing SKS cluster

## Categories
containers, system

## Links
- More information: https://community.exoscale.com/product/compute/containers/
- tldr page: https://tldr.inbrowser.app/pages/common/exo-compute-sks
