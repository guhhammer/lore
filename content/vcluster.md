# vcluster

Create and manage lightweight virtual Kubernetes clusters in namespaces.

## Usage
- `vcluster create vcluster_name -n namespace` : create a virtual cluster in a specific namespace
- `vcluster connect vcluster_name -n namespace --local-port port --insecure` : connect to a virtual cluster with a local port and insecure mode
- `vcluster list` : list all virtual clusters
- `vcluster delete vcluster_name` : delete a virtual cluster
- `vcluster platform list` : list platform-managed virtual clusters
- `vcluster platform create vcluster_name -n namespace` : create a platform-managed virtual cluster
- `vcluster platform connect vcluster_name -n namespace` : connect to a platform-managed virtual cluster
- `vcluster platform delete vcluster_name -n namespace` : delete a platform-managed virtual cluster

## Categories
containers, network

## Links
- More information: https://www.vcluster.com/docs/vcluster
- tldr page: https://tldr.inbrowser.app/pages/common/vcluster
