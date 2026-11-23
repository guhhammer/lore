# flux

Continuous and progressive delivery solution for Kubernetes.

## Usage
- `flux check --pre` : check if the Flux prerequisites are met on the cluster
- `flux bootstrap github|gitlab --owner owner --repository repository --path path/to/cluster_directory` : bootstrap Flux on a Kubernetes cluster
- `flux create source git source_name --url https://github.com/repository --branch branch_name` : create a new GitRepository source
- `flux get all` : list all Flux custom resources
- `flux reconcile kustomization name --with-source` : trigger a reconciliation for a specific Kustomization and apply changes from source
- `flux suspend kustomization name` : suspend reconciliation for a Kustomization
- `flux resume kustomization name` : resume reconciliation for a Kustomization
- `flux subcommand --help` : display help for a specific subcommand

## Categories
containers, network, packages

## Links
- More information: https://fluxcd.io/flux/cmd/
- tldr page: https://tldr.inbrowser.app/pages/common/flux
