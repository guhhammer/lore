# kubectl attach

Attach to a process that is already running inside an existing container.

## Usage
- `kubectl attach pod_name` : get output from a pod
- `kubectl attach pod_name -c container_name` : get output from a container in the specified pod
- `kubectl attach rs/replicaset_name` : get output from the first pod of a replica set
- `kubectl attach pod_name -it` : create an interactive session with a pod
- `kubectl attach pod_name -c container_name -it` : create an interactive session with a specific container from a pod

## Categories
containers, processes

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_attach/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-attach
