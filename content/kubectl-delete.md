# kubectl delete

Delete Kubernetes resources.

## Usage
- `kubectl delete po pod_name` : delete a specific pod
- `kubectl delete deploy deployment_name` : delete a specific deployment
- `kubectl delete no node_name` : delete a specific node
- `kubectl delete po --all -n namespace` : delete all pods in a specified namespace
- `kubectl delete deploy,svc --all -n namespace` : delete all deployments and services in a specified namespace
- `kubectl delete no --all` : delete all nodes
- `kubectl delete -f path/to/manifest.yaml` : delete resources defined in a YAML manifest

## Categories
containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_delete/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-delete
