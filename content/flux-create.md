# flux create

Create or update Flux resources.

## Usage
- `flux create source git source_name --url https://github.com/repository --branch branch_name` : create a GitRepository source
- `flux create kustomization kustomization_name --source source_name --path path/to/directory` : create a Kustomization resource to sync a directory from a Git source
- `flux create helmrelease release_name --chart chart_name --source HelmRepository/repository_name` : create a HelmRelease
- `flux create secret git secret_name --url https://github.com/repository --username "username" --password "password"` : create a Secret to authenticate with a Git repository

## Categories
network, version-control, files

## Links
- More information: https://fluxcd.io/flux/cmd/flux_create/
- tldr page: https://tldr.inbrowser.app/pages/common/flux-create
