# flux delete

Delete sources and resources in Flux.

## Usage
- `flux delete kustomization kustomization_name` : delete a Kustomization
- `flux delete kustomization kustomization_name -s` : delete a Kustomization without asking for confirmation
- `flux delete helmrelease release_name` : delete a HelmRelease
- `flux delete source git source_name` : delete a GitRepository source
- `flux delete source helm source_name` : delete a HelmRepository source

## Links
- More information: https://fluxcd.io/flux/cmd/flux_delete/
- tldr page: https://tldr.inbrowser.app/pages/common/flux-delete
