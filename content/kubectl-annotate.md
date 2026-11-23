# kubectl annotate

Annotate Kubernetes resources.

## Usage
- `kubectl annotate po pod_name key=value` : annotate a pod
- `kubectl annotate po pod_name key=value --overwrite` : update a pod annotation by overwriting the existing value
- `kubectl annotate po key=value -l label_key=label_value` : annotate all pods in a namespace with a specific label selector
- `kubectl annotate po pod_name --list` : list all annotations a pod has
- `kubectl annotate po pod_name key-` : remove the annotation from a pod

## Categories
containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_annotate/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-annotate
