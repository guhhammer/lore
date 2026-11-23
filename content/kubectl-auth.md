# kubectl auth

Inspect access permissions in a Kubernetes cluster.

## Usage
- `kubectl auth can-i '*' '*' -n namespace` : check if the current user can perform all actions on all resources in a specific namespace
- `kubectl auth can-i verb resource -n namespace` : check if the current user can perform a specific verb on a specific resource
- `kubectl auth can-i verb resource -n namespace --as user_or_sa` : check if a specific user or service account can perform an action on a resource
- `kubectl auth can-i --list -n namespace` : list all actions the current user is allowed to perform in a namespace

## Categories
containers, users

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_auth/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-auth
