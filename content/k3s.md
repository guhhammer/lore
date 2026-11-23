# k3s

Install and manage lightweight Kubernetes clusters.

## Usage
- `k3s kubectl get nodes` : run the embedded `kubectl` command
- `k3s etcd-snapshot save` : take an etcd snapshot of the cluster
- `k3s certificate rotate-ca` : rotate the CA certificate
- `k3s token list` : manage bootstrap tokens
- `k3s-uninstall.sh` : uninstall K3s and remove all components

## Categories
containers, packages

## Links
- More information: https://docs.k3s.io/cli
- tldr page: https://tldr.inbrowser.app/pages/common/k3s
