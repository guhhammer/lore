# kubectl top

See the resource consumption for nodes or pods.

## Usage
- `kubectl top no` : get the resource consumption of all nodes
- `kubectl top no node_name` : get resource consumption of a specific node
- `kubectl top po` : get resource consumption of all pods
- `kubectl top po pod_name` : get resource consumption of a specific pod
- `kubectl top po -n namespace_name` : get resource consumption of all pods in a namespace
- `kubectl top po --containers` : get resource consumption of all containers in a pod
- `kubectl top po -l key=value` : get resource consumption of all pods with the specified label

## Categories
containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_top/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-top
