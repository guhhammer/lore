# kubectl rollout

Manage the rollout of a Kubernetes resource (deployments, daemonsets, and statefulsets).

## Usage
- `kubectl rollout restart resource_type/resource_name` : start a rolling restart of a resource
- `kubectl rollout status resource_type/resource_name` : watch the rolling update status of a resource
- `kubectl rollout undo resource_type/resource_name` : roll back a resource to the previous revision
- `kubectl rollout history resource_type/resource_name` : view the rollout history of a resource

## Categories
containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_rollout/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-rollout
