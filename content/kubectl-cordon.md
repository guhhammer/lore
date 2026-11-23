# kubectl cordon

Mark a node as unschedulable, preventing new pods from being assigned to it.

## Usage
- `kubectl cordon node_name` : mark a node as unschedulable
- `kubectl cordon node_name1 node_name2 ...` : mark multiple nodes as unschedulable
- `kubectl cordon node_name --context context_name` : mark a node as unschedulable in a specific context
- `kubectl cordon -l label_key=label_value` : mark nodes matching a label selector as unschedulable
- `kubectl cordon node_name --dry-run=none|server|client` : simulate the changes without actually cordoning the nodes

## See also
- kubectl uncordon

## Categories
containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_cordon/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-cordon
