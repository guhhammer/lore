# kubectl drain

Drain a node in preparation for maintenance by marking it unschedulable and evicting all pods.

## Usage
- `kubectl drain node_name` : drain a node
- `kubectl drain node_name --ignore-daemonsets` : drain a node, ignoring DaemonSet-managed pods
- `kubectl drain node_name --ignore-daemonsets --delete-emptydir-data` : drain a node and delete pods using emptyDir volumes (local data will be lost)
- `kubectl drain node_name --force` : drain a node, forcing eviction of pods not managed by a controller
- `kubectl drain node_name --grace-period seconds` : drain a node with a custom grace period for pod termination
- `kubectl drain node_name --pod-selector label_key=label_value` : drain a node, evicting only pods that match a label selector
- `kubectl drain node_name --timeout duration` : drain a node with a timeout
- `kubectl drain node_name --dry-run=none|server|client` : simulate the drain operation without actually evicting pods

## See also
- kubectl cordon
- kubectl uncordon

## Categories
containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_drain/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-drain
