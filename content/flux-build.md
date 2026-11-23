# flux build

Build Flux manifests.

## Usage
- `flux build kustomization name --path path/to/kustomization` : build a Kustomization
- `flux build kustomization name --path path/to/kustomization >path/to/manifests.yaml` : build a Kustomization and output the manifests to a file
- `flux build kustomization name --path path/to/kustomization | kubectl apply -f -` : build a Kustomization and pipe it directly to `kubectl`

## Categories
development, files

## Links
- More information: https://fluxcd.io/flux/cmd/flux_build/
- tldr page: https://tldr.inbrowser.app/pages/common/flux-build
