# flux resume

Resume suspended Flux resources.

## Usage
- `flux resume kustomization kustomization_name` : resume reconciliation for a Kustomization
- `flux resume kustomization --all` : resume reconciliation for all Kustomizations in a namespace
- `flux resume helmrelease release_name` : resume reconciliation for a HelmRelease
- `flux resume source git source_name` : resume reconciliation for a GitRepository source
- `flux resume image update automation_name` : resume reconciliation for an ImageUpdateAutomation resource

## Links
- More information: https://fluxcd.io/flux/cmd/flux_resume/
- tldr page: https://tldr.inbrowser.app/pages/common/flux-resume
