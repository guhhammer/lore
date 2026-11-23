# flux export

Export Flux resources in YAML format.

## Usage
- `flux export kustomization kustomization_name` : export a Kustomization
- `flux export kustomization --all >path/to/kustomizations.yaml` : export all Kustomization resources and save them to a file
- `flux export source git source_name --with-credentials` : export a GitRepository source including credentials
- `flux export source git --all` : export all GitRepository sources
- `flux export helmrelease release_name` : export a HelmRelease

## Categories
text

## Links
- More information: https://fluxcd.io/flux/cmd/flux_export/
- tldr page: https://tldr.inbrowser.app/pages/common/flux-export
