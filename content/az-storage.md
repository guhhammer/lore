# az storage

Manage Azure Cloud Storage resources. Part of `azure-cli` (also known as `az`).

## Usage
- `az storage account create -g group_name -n account_name -l location --sku account_sku` : create a storage account specifying a location
- `az storage account list -g group_name` : list all storage accounts in a resource group
- `az storage account keys list -g group_name -n account_name` : list the access keys for a storage account
- `az storage account delete -g group_name -n account_name` : delete a storage account
- `az storage account update --min-tls-version TLS1_0|TLS1_1|TLS1_2 -g group_name -n account_name` : update the minimum tls version setting for a storage account

## Categories
cloud, users

## Links
- More information: https://learn.microsoft.com/cli/azure/storage
- tldr page: https://tldr.inbrowser.app/pages/common/az-storage
