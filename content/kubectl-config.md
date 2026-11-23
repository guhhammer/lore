# kubectl config

Manage Kubernetes configuration (kubeconfig) files for accessing clusters via `kubectl` or the Kubernetes API. By default, the Kubernetes will get its configuration from `${HOME}/.kube/config`.

## Usage
- `kubectl config get-contexts` : get all contexts in the default kubeconfig file
- `kubectl config get-clusters|get-contexts|get-users --kubeconfig path/to/kubeconfig.yaml` : get all clusters/contexts/users in a custom kubeconfig file
- `kubectl config current-context` : get the current context
- `kubectl config set-context --current --namespace namespace` : set the default namespace of the current context
- `kubectl config use context_name` : switch to another context
- `kubectl config delete-cluster|delete-context|delete-user cluster|context|user` : delete clusters/contexts/users
- `export KUBECONFIG="path/to/kubeconfig1.yaml:path/to/kubeconfig2.yaml"` : permanently add custom kubeconfig files

## See also
- kubectx
- kubens

## Categories
containers, files, users

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_config/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-config
