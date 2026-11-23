# az storage blob

Manage blob storage containers and objects in Azure. Part of `azure-cli` (also known as `az`).

## Usage
- `az storage blob download --account-name account_name --account-key account_key -c container_name -n blob_name -f path/to/file` : download a blob to a file path specifying a source container
- `az storage blob download-batch --account-name account_name --account-key account_key -s container_name --pattern filename_regex -d path/to/destination` : download blobs from a blob container recursively
- `az storage blob upload --account-name account_name --account-key account_key -c container_name -n blob_name -f path/to/file` : upload a local file to blob storage
- `az storage blob delete --account-name account_name --account-key account_key -c container_name -n blob_name` : delete a blob object
- `az storage blob generate-sas --account-name account_name --account-key account_key -c container_name -n blob_name --permissions permission_set --expiry Y-m-d'T'H:M'Z' --https-only` : generate a shared access signature for a blob

## Categories
users, network, files

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/blob
- tldr page: https://tldr.inbrowser.app/pages/common/az-storage-blob
