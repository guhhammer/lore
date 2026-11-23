# az aks

Manage Azure Kubernetes Service (AKS) clusters. Part of `azure-cli` (also known as `az`).

## Usage
- `az aks list -g resource_group` : list AKS clusters
- `az aks create -g resource_group -n name -c count --node-vm-size size` : create a new AKS cluster
- `az aks delete -g resource_group -n name` : delete an AKS cluster
- `az aks get-credentials -g resource_group -n name` : get the access credentials for an AKS cluster
- `az aks get-upgrades -g resource_group -n name` : get the upgrade versions available for an AKS cluster

## Categories
cloud, containers, system

## Links
- More information: https://learn.microsoft.com/cli/azure/aks
- tldr page: https://tldr.inbrowser.app/pages/common/az-aks
