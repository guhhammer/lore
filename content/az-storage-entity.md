# az storage entity

Manage Azure Table storage entities. Part of `azure-cli` (also known as `az`).

## Usage
- `az storage entity insert -e space_separated_key_value_pairs -t table_name --account-name storage_account_name --account-key storage_account_key` : insert an entity into a table
- `az storage entity delete --partition-key partition_key --row-key row_key -t table_name --account-name storage_account_name --account-key storage_account_key` : delete an existing entity from a table
- `az storage entity merge -e space_separated_key_value_pairs -t table_name --account-name storage_account_name --account-key storage_account_key` : update an existing entity by merging its properties
- `az storage entity query --filter query_filter -t table_name --account-name storage_account_name --account-key storage_account_key` : list entities which satisfy a query
- `az storage entity show --partition-key partition_key --row-key row_key -t table_name --account-name storage_account_name --account-key storage_account_key` : get an entity from the specified table

## Categories
users, databases, cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/entity
- tldr page: https://tldr.inbrowser.app/pages/common/az-storage-entity
