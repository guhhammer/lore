# kubectl uncordon

Mark a node as schedulable, allowing new pods to be assigned to it.

## Usage
- `kubectl uncordon node_name` : mark a node as schedulable
- `kubectl uncordon node_name1 node_name2 ...` : mark multiple nodes as schedulable
- `kubectl uncordon node_name --context context_name` : mark a node as schedulable in a specific context
- `kubectl uncordon -l label_key=label_value` : mark nodes matching a label selector as schedulable
- `kubectl uncordon node_name --dry-run=none|server|client` : simulate the changes without actually uncordoning the nodes

## See also
- kubectl cordon

## Categories
containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_uncordon/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-uncordon
