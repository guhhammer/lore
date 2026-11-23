# helm

A package manager for Kubernetes. Some subcommands such as `install` have their own usage documentation.

## Usage
- `helm create chart_name` : create a helm chart
- `helm repo add repository_name` : add a new helm repository
- `helm repo ls` : list helm repositories
- `helm repo up` : update helm repositories
- `helm repo rm repository_name` : delete a helm repository
- `helm install name repository_name/chart_name` : install a helm chart
- `helm get chart_release_name` : download helm chart as a `.tar` archive
- `helm dep up` : update helm dependencies

## Categories
containers

## Links
- More information: https://helm.sh/docs/helm/
- tldr page: https://tldr.inbrowser.app/pages/common/helm
