# kubectl edit

Edit Kubernetes resources.

## Usage
- `kubectl edit po/pod_name` : edit a pod in the default namespace
- `kubectl edit deploy/deployment_name` : edit a deployment in the default namespace
- `kubectl edit svc/service_name` : edit a service in the default namespace
- `kubectl edit resource -n namespace` : edit all entries of a given resource in a given namespace
- `KUBE_EDITOR=nano kubectl edit resource/resource_name` : edit a resource using a specific editor
- `kubectl edit resource/resource_name -o json` : edit a resource in JSON format

## Categories
containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_edit/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-edit
