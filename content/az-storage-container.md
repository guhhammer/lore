# az storage container

Manage blob storage containers in Azure. Part of `azure-cli` (also known as `az`).

## Usage
- `az storage container create --account-name storage_account_name -n container_name --public-access access_level --fail-on-exist` : create a container in a storage account
- `az storage container generate-sas --account-name storage_account_name -n container_name --permissions sas_permissions --expiry expiry_date --https-only` : generate a shared access signature for the container
- `az storage container list --account-name storage_account_name --prefix filter_prefix` : list containers in a storage account
- `az storage container delete --account-name storage_account_name -n container_name --fail-not-exist` : mark the specified container for deletion

## Categories
containers, cloud, users

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/container
- tldr page: https://tldr.inbrowser.app/pages/common/az-storage-container
