# kubeadm

Interface for creating and managing Kubernetes clusters.

## Usage
- `kubeadm init` : create a Kubernetes control plane
- `kubeadm join --token token` : bootstrap a Kubernetes worker node and join it to a cluster
- `kubeadm token create --ttl 12h0m0s` : create a new bootstrap token with a TTL of 12 hours
- `kubeadm upgrade plan` : check if the Kubernetes cluster is upgradeable and which versions are available
- `kubeadm upgrade apply version` : upgrade Kubernetes cluster to a specified version
- `kubeadm config view` : view the kubeadm ConfigMap containing the cluster's configuration
- `kubeadm reset` : revert changes made to the host by 'kubeadm init' or 'kubeadm join'

## Categories
containers, network, packages

## Links
- More information: https://kubernetes.io/docs/reference/setup-tools/kubeadm/
- tldr page: https://tldr.inbrowser.app/pages/common/kubeadm
