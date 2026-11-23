# kubectl set

Update fields of existing application resources.

## Usage
- `kubectl set image deploy/deployment_name container_name=image` : update the image of a container in a deployment
- `kubectl set image deploy/deployment_name *=image` : update the image of all containers in a deployment
- `kubectl set resources deploy/deployment_name --limits cpu=cpu_limit,memory=memory_limit` : set resource limits (CPU and memory) on a deployment
- `kubectl set env deploy/deployment_name variable_name=value` : set an environment variable on a deployment
- `kubectl set env deploy/deployment_name variable_name-` : remove an environment variable from a deployment
- `kubectl set env --from secret|configmap/resource_name deploy/deployment_name` : import environment variables from a secret or ConfigMap
- `kubectl set sa deploy/deployment_name service_account_name` : update the service account of a deployment

## Categories
containers, media

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_set/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-set
