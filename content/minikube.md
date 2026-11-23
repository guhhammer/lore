# minikube

Run Kubernetes locally.

## Usage
- `minikube start` : start the cluster
- `minikube ip` : get the IP address of the cluster
- `minikube service my_service --url` : access a service named `my_service` exposed via a node port and get the URL
- `minikube dashboard` : open the Kubernetes dashboard in a browser
- `minikube stop` : stop the running cluster
- `minikube delete` : delete the cluster
- `minikube tunnel` : connect to LoadBalancer services

## Categories
containers, network, system

## Links
- More information: https://minikube.sigs.k8s.io/docs/
- tldr page: https://tldr.inbrowser.app/pages/common/minikube
