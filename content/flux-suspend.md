# flux suspend

Suspend Flux resources.

## Usage
- `flux suspend kustomization kustomization_name` : suspend reconciliation for a Kustomization
- `flux suspend kustomization --all` : suspend reconciliation for all Kustomizations in a namespace
- `flux suspend helmrelease release_name` : suspend reconciliation for a HelmRelease
- `flux suspend source git source_name` : suspend reconciliation for a GitRepository source
- `flux suspend image update automation_name` : suspend reconciliation for an ImageUpdateAutomation resource

## Links
- More information: https://fluxcd.io/flux/cmd/flux_suspend/
- tldr page: https://tldr.inbrowser.app/pages/common/flux-suspend
