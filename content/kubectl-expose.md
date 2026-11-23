# kubectl expose

Expose a resource as a new Kubernetes service.

## Usage
- `kubectl expose resource_type resource_name --port node_port --target-port container_port` : create a service for a resource, which will be served from container port to node port
- `kubectl expose -f path/to/file.yml --port node_port --target-port container_port` : create a service for a resource identified by a file
- `kubectl expose resource_type resource_name --port node_port --name service_name` : create a service with a name, to serve to a node port which will be same for container port

## Categories
network, containers, system

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_expose/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-expose
