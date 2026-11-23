# k3d

A wrapper to easily create k3s clusters inside Docker.

## Usage
- `k3d cluster create cluster_name` : create a cluster
- `k3d cluster list` : list all clusters
- `k3d cluster list -o json|yaml` : list clusters in JSON or YAML format
- `k3d cluster list --no-headers` : list clusters without headers
- `k3d cluster delete cluster_name` : delete a cluster
- `k3d node create node_name` : create a new containerized k3s node
- `k3d image import image_name -c cluster_name` : import an image from Docker into a k3d cluster
- `k3d registry create registry_name` : create a new registry

## Categories
containers, text

## Links
- More information: https://k3d.io/stable/usage/commands/
- tldr page: https://tldr.inbrowser.app/pages/common/k3d
