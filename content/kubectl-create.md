# kubectl create

Create a resource from a file or from `stdin`.

## Usage
- `kubectl create -f path/to/file.yml` : create a resource using the resource definition file
- `kubectl create -f -` : create a resource from `stdin`
- `kubectl create deploy deployment_name --image image` : create a deployment
- `kubectl create deploy deployment_name --image image --replicas number_of_replicas` : create a deployment with replicas
- `kubectl create svc service_type service_name --tcp port:target_port` : create a service
- `kubectl create ns namespace_name` : create a namespace

## Categories
files, containers, media

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_create/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-create
