# rbac-lookup

Find roles and cluster roles attached to any user, service account, or group name in your Kubernetes cluster.

## Usage
- `rbac-lookup` : view all RBAC bindings
- `rbac-lookup search_term` : view RBAC bindings that match a given expression
- `rbac-lookup -o wide` : view all RBAC bindings along with the source role binding
- `rbac-lookup -k user|group|serviceaccount` : view all RBAC bindings filtered by subject
- `rbac-lookup --gke` : view all RBAC bindings along with IAM roles (if you are using GKE)

## Categories
users, containers, system

## Links
- More information: https://github.com/FairwindsOps/rbac-lookup
- tldr page: https://tldr.inbrowser.app/pages/common/rbac-lookup
