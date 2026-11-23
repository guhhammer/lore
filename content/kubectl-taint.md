# kubectl taint

Update the taints on nodes.

## Usage
- `kubectl taint no node_name label_key=label_value:effect` : apply taint to a node
- `kubectl taint no node_name label_key:effect-` : remove taint from a node
- `kubectl taint no node_name label_key-` : remove all taints from a node

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_taint/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-taint
