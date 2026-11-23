# kubectl apply

Manage applications through files defining Kubernetes resources. Create and update resources in a cluster.

## Usage
- `kubectl apply -f path/to/file` : apply a configuration to a resource by file name
- `kubectl apply -k path/to/directory` : apply a configuration to a resource from `kustomization.yaml` in a directory
- `cat pod.json | kubectl apply -f -` : apply a configuration to a resource by `stdin`
- `kubectl apply edit-last-applied -f path/to/file` : edit the latest last-applied-configuration annotations of resources from the default editor
- `kubectl apply set-last-applied -f path/to/file` : set the latest last-applied-configuration annotations by setting it to match the contents of a file
- `kubectl apply view-last-applied -f path/to/file` : view the latest last-applied-configuration annotations by type/name or file

## Categories
files, containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_apply/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-apply
