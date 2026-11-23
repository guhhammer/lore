# kubectl

Run commands against Kubernetes clusters. Some subcommands such as `run` have their own usage documentation.

## Usage
- `kubectl get pods|service|deployment|ingress|... -o wide` : list information about a resource with more details
- `kubectl label pods name unhealthy=true` : update specified pod with the label `unhealthy` and the value `true`
- `kubectl get all` : list all resources with different types
- `kubectl top pods|nodes` : display resource (CPU/Memory/Storage) usage of nodes or pods
- `kubectl cluster-info` : print the address of the master and cluster services
- `kubectl explain pods.spec.containers` : display an explanation of a specific field
- `kubectl logs pod_name` : print the logs for a container in a pod or specified resource
- `kubectl exec pod_name -- ls /` : run command in an existing pod

## Categories
containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl
