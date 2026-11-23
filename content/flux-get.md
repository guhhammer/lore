# flux get

Display the status of Flux resources.

## Usage
- `flux get all -A` : list all resources and statuses across all namespaces
- `flux get kustomizations` : list all Kustomizations and their statuses
- `flux get kustomizations --show-source` : list all Kustomizations with source information
- `flux get helmreleases` : list all HelmReleases
- `flux get sources git` : list all GitRepository sources
- `flux get kustomizations -w` : watch Kustomizations for changes

## Categories
desktop

## Links
- More information: https://fluxcd.io/flux/cmd/flux_get/
- tldr page: https://tldr.inbrowser.app/pages/common/flux-get
