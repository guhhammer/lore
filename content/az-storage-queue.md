# az storage queue

Manage storage queues in Azure. Part of `azure-cli` (also known as `az`).

## Usage
- `az storage queue create --account-name storage_account_name -n queue_name --metadata queue_metadata` : create a queue
- `az storage queue generate-sas --account-name storage_account_name -n queue_name --permissions queue_permissions --expiry expiry_date --https-only` : generate a shared access signature for the queue
- `az storage queue list --prefix filter_prefix --account-name storage_account_name` : list queues in a storage account
- `az storage queue delete --account-name storage_account_name -n queue_name --fail-not-exist` : delete the specified queue and any messages it contains

## Categories
cloud, users, network

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/queue
- tldr page: https://tldr.inbrowser.app/pages/common/az-storage-queue
