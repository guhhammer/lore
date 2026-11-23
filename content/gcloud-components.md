# gcloud components

Manage Google Cloud CLI components.

## Usage
- `gcloud components list` : view available components for installation
- `gcloud components install component_id1 component_id2 ...` : install one or more components (installs any dependencies as well)
- `gcloud components update` : update all components to the latest version
- `gcloud components update --version=1.2.3` : update all components to a specific version
- `gcloud components update --quiet` : update components without confirmation (useful for automation scripts)

## Categories
cloud, packages

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/components
- tldr page: https://tldr.inbrowser.app/pages/common/gcloud-components
