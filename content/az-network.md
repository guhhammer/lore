# az network

Manage Azure Network resources. Part of `azure-cli` (also known as `az`).

## Usage
- `az network list-usages` : list network resources in a region that are used against a subscription quota
- `az network vnet list` : list all virtual networks in a subscription
- `az network vnet create --address-prefixes 10.0.0.0/16 -n vnet -g group_name --subnet-name subnet --subnet-prefixes 10.0.0.0/24` : create a virtual network
- `az network nic update --accelerated-networking true -n nic -g resource_group` : enable accelerated networking for a network interface card

## Categories
network, cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/network
- tldr page: https://tldr.inbrowser.app/pages/common/az-network
