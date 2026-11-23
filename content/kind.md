# kind

Run local Kubernetes clusters using Docker container "nodes". Designed for testing Kubernetes itself, but may be used for local development or continuous integration.

## Usage
- `kind create cluster --name cluster_name` : create a local Kubernetes cluster
- `kind delete clusters cluster_name` : delete one or more clusters
- `kind get clusters|nodes|kubeconfig` : get details about clusters, nodes, or the kubeconfig
- `kind export kubeconfig|logs` : export the kubeconfig or the logs

## Categories
containers

## Links
- More information: https://github.com/kubernetes-sigs/kind
- tldr page: https://tldr.inbrowser.app/pages/common/kind
