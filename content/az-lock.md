# az lock

Manage Azure locks. Part of `azure-cli` (also known as `az`).

## Usage
- `az lock create -n lock_name -t ReadOnly` : create a read-only subscription level lock
- `az lock create -n lock_name -g group_name -t ReadOnly` : create a read-only resource group level lock
- `az lock delete -n lock_name` : delete a subscription level lock
- `az lock delete -n lock_name -g group_name` : delete a resource group level lock
- `az lock list` : list out all locks on the subscription level
- `az lock show -n lock_name` : show a subscription level lock with a specific name

## Categories
cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/lock
- tldr page: https://tldr.inbrowser.app/pages/common/az-lock
