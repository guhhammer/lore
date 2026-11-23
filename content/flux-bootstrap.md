# flux bootstrap

Bootstrap Flux on a Kubernetes cluster.

## Usage
- `flux bootstrap github --owner owner --repository repository --path path/to/cluster_directory --personal` : bootstrap using a GitHub repository
- `flux bootstrap gitlab --owner owner --repository repository --path path/to/cluster_directory` : bootstrap using a GitLab repository
- `flux bootstrap git --url ssh://git@example.com/repository.git --branch main --path path/to/cluster_directory` : bootstrap from a generic Git repository via SSH

## Categories
packages, containers, files

## Links
- More information: https://fluxcd.io/flux/cmd/flux_bootstrap/
- tldr page: https://tldr.inbrowser.app/pages/common/flux-bootstrap
