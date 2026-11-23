# azcopy

Copy data to and from Azure Storage.

## Usage
- `azcopy login` : log in to an Azure Tenant
- `azcopy c 'path/to/source_file' 'https://storage_account_name.blob.core.windows.net/container_name/blob_name'` : upload a local file
- `azcopy c 'path/to/source_directory' 'https://storage_account_name.blob.core.windows.net/container_name' --include-pattern '*.txt;*.jpg'` : upload files with `.txt` and `.jpg` extensions
- `azcopy c 'https://source_storage_account_name.blob.core.windows.net/container_name' 'https://destination_storage_account_name.blob.core.windows.net/container_name'` : copy a container directly between two Azure storage accounts
- `azcopy s 'path/to/source_directory' 'https://storage_account_name.blob.core.windows.net/container_name' --delete-destination true` : synchronize a local directory and delete files in the destination if they no longer exist in the source
- `azcopy -h` : display help

## See also
- az storage

## Categories
files, network, cloud

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/azcopy
