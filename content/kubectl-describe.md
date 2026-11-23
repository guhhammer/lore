# kubectl describe

Show details of Kubernetes resources.

## Usage
- `kubectl describe po -n namespace` : show details of pods in a namespace
- `kubectl describe no -n namespace` : show details of nodes in a namespace
- `kubectl describe po pod_name -n namespace` : show the details of a specific pod in a namespace
- `kubectl describe no node_name -n namespace` : show the details of a specific node in a namespace
- `kubectl describe -f path/to/manifest.yaml` : show details of Kubernetes objects defined in a YAML manifest file

## Categories
containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_describe/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-describe
