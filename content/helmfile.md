# helmfile

Declare and manage Helm releases and related Kubernetes manifests.

## Usage
- `helmfile init` : install required Helm plugins
- `helmfile diff` : preview the changes that would be applied to the cluster
- `helmfile apply` : apply releases defined in `helmfile.yaml` to the cluster
- `helmfile sync` : synchronize the cluster with the desired state, force-recreating if needed
- `helmfile -e dev|production|staging apply` : apply releases for a specific environment
- `helmfile destroy` : destroy all releases defined in `helmfile.yaml`
- `helmfile -h` : display help
- `helmfile version` : display version

## Categories
containers

## Links
- More information: https://helmfile.readthedocs.io
- tldr page: https://tldr.inbrowser.app/pages/common/helmfile
