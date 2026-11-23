# az storage account

Manage storage accounts in Azure. Part of `azure-cli` (also known as `az`).

## Usage
- `az storage account create -n storage_account_name -g azure_resource_group --location azure_location --sku storage_account_sku` : create a storage account
- `az storage account generate-sas --account-name storage_account_name -n account_name --permissions sas_permissions --expiry expiry_date --services storage_services --resource-types resource_types` : generate a shared access signature for a specific storage account
- `az storage account list -g azure_resource_group` : list storage accounts
- `az storage account delete -n storage_account_name -g azure_resource_group` : delete a specific storage account

## Categories
users, cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/account
- tldr page: https://tldr.inbrowser.app/pages/common/az-storage-account
