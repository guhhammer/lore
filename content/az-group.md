# az group

Manage resource groups and template deployments. Part of `azure-cli` (also known as `az`).

## Usage
- `az group create -n name -l location` : create a new resource group
- `az group exists -n name` : check if a resource group exists
- `az group delete -n name` : delete a resource group
- `az group wait -n name --created|deleted|exists|updated` : wait until a condition of the resource group is met

## Categories
users, cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/group
- tldr page: https://tldr.inbrowser.app/pages/common/az-group
