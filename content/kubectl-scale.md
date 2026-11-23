# kubectl scale

Set a new size for a deployment, replica set, replication controller, or stateful set.

## Usage
- `kubectl scale --replicas replicas_count rs/replica_name` : scale a replica set
- `kubectl scale --replicas replicas_count -f path/to/file.yml` : scale a resource identified by a file
- `kubectl scale --replicas replicas_count --current-replicas current_replicas deploy/deployment_name` : scale a deployment based on current number of replicas

## Categories
files

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_scale/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-scale
