# stern

Tail multiple pods and containers from Kubernetes.

## Usage
- `stern .` : tail all pods within a current namespace
- `stern . --container-state running|waiting|terminated` : tail all pods with a specific status
- `stern pod_query` : tail all pods that matches a given `regex`
- `stern pod_query -A` : tail matched pods from all namespaces
- `stern pod_query -s 15m` : tail matched pods from 15 minutes ago
- `stern pod_query -l release=canary` : tail matched pods with a specific label

## Categories
containers

## Links
- More information: https://github.com/stern/stern#usage
- tldr page: https://tldr.inbrowser.app/pages/common/stern
