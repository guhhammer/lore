# flux reconcile

Trigger a reconciliation of Flux sources and resources.

## Usage
- `flux reconcile source git source_name` : trigger reconciliation of a GitRepository source
- `flux reconcile kustomization kustomization_name` : trigger reconciliation of a Kustomization
- `flux reconcile kustomization kustomization_name --with-source` : trigger reconciliation of a Kustomization and its source
- `flux reconcile helmrelease release_name` : trigger reconciliation of a HelmRelease
- `flux reconcile helmrelease release_name --with-source` : trigger reconciliation of a HelmRelease and its source

## Links
- More information: https://fluxcd.io/flux/cmd/flux_reconcile/
- tldr page: https://tldr.inbrowser.app/pages/common/flux-reconcile
