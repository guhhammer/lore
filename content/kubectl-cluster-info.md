# kubectl cluster-info

Display endpoint information about the Kubernetes master and services in the cluster.

## Usage
- `kubectl cluster-info` : show basic cluster information
- `kubectl cluster-info dump` : dump current cluster state to `stdout` (for debugging)
- `kubectl cluster-info dump --output-directory path/to/directory` : dump cluster state to a directory
- `kubectl cluster-info --context context_name` : use a specific kubeconfig context

## Categories
containers, files, desktop

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_cluster-info/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-cluster-info
