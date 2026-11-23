# kubectl kustomize

Build a set of Kubernetes resources using a `kustomization.yaml` file.

## Usage
- `kubectl kustomize` : build resources from the current directory
- `kubectl kustomize path/to/directory` : build resources from a specific directory
- `kubectl kustomize https://github.com/user/repo/path` : build resources from a remote URL
- `kubectl kustomize path/to/directory >output.yaml` : build resources and save to a file
- `kubectl kustomize --load-restrictor LoadRestrictionsNone path/to/directory` : build resources with load restrictor disabled

## Categories
files, development, containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_kustomize/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-kustomize
