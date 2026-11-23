# kubectl get

Get Kubernetes objects and resources.

## Usage
- `kubectl get ns` : get all namespaces in the current cluster
- `kubectl get no -n namespace` : get nodes in a specified namespace
- `kubectl get po -n namespace` : get pods in a specified namespace
- `kubectl get deploy -n namespace` : get deployments in a specified namespace
- `kubectl get svc -n namespace` : get services in a specified namespace
- `kubectl get persistentvolumeclaims|secret|...` : get other resources
- `kubectl get all -A` : get all resources in all namespaces
- `kubectl get -f path/to/manifest.yaml` : get Kubernetes objects defined in a YAML manifest file

## Categories
containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_get/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-get
