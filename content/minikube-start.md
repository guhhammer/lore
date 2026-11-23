# minikube start

Start `minikube` with different configurations.

## Usage
- `minikube start --kubernetes-version v1.24.0` : start `minikube` with a specific Kubernetes version
- `minikube start --memory 2048 --cpus 2` : start `minikube` with specific resource allocations (e.g., memory and CPU)
- `minikube start --driver virtualbox` : start `minikube` with a specific driver (e.g., VirtualBox)
- `minikube start --background` : start `minikube` in the background (headless mode)
- `minikube start --addons metrics-server` : start `minikube` with custom add-ons (e.g., the metrics server)

## Links
- More information: https://minikube.sigs.k8s.io/docs/commands/start/
- tldr page: https://tldr.inbrowser.app/pages/common/minikube-start
