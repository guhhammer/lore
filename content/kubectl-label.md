# kubectl label

Label Kubernetes resources.

## Usage
- `kubectl label po pod_name key=value` : label a pod
- `kubectl label po pod_name key=value --overwrite` : update a pod label by overwriting the existing value
- `kubectl label po key=value --all` : label all pods in the namespace
- `kubectl label -f pod_definition_file key=value` : label a pod identified by the pod definition file
- `kubectl label po pod_name key-` : remove the label from a pod

## Categories
containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_label/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-label
