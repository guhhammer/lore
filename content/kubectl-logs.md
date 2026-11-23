# kubectl logs

Show logs for containers in a pod.

## Usage
- `kubectl logs pod_name` : show logs for a single-container pod
- `kubectl logs -c container_name pod_name` : show logs for a specified container in a pod
- `kubectl logs --all-containers=true pod_name` : show logs for all containers in a pod
- `kubectl logs -f pod_name` : stream pod logs
- `kubectl logs --since relative_time pod_name` : show pod logs newer than a relative time like `10s`, `5m`, or `1h`
- `kubectl logs --tail 10 pod_name` : show the 10 most recent logs in a pod
- `kubectl logs deploy/deployment_name` : show all pod logs for a given deployment

## Categories
containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_logs/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-logs
