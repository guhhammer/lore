# kubectl port-forward

Forward one or more local ports to a pod.

## Usage
- `kubectl port-forward po/pod_name 5000 6000` : forward local ports 5000 and 6000 to the pod ports 5000 and 6000
- `kubectl port-forward po/pod_name :5000` : forward a random local port to the pod port 5000
- `kubectl port-forward deploy/deployment_name 5000 6000` : forward local ports 5000 and 6000 to the deployment ports 5000 and 6000
- `kubectl port-forward svc/service_name 8443:https` : forward local port 8443 to the service port named https
- `kubectl port-forward po/pod_name 8888:5000 --address 0.0.0.0` : forward port 8888 on all addresses to the pod port 5000
- `kubectl port-forward po/pod_name 8888:5000 --address localhost,10.19.21.23` : forward port 8888 on localhost and selected IP to the pod port 5000

## Categories
network, containers

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_port-forward/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-port-forward
