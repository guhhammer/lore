# az disk

Manage Azure Managed Disks. Part of `azure-cli` (also known as `az`).

## Usage
- `az disk create -g resource_group -n disk_name -z size_in_gb` : create a managed disk
- `az disk list -g resource_group` : list managed disks in a resource group
- `az disk delete -g resource_group -n disk_name` : delete a managed disk
- `az disk grant-access -g resource_group -n disk_name --access Read|Write --duration-in-seconds seconds` : grant read or write access to a managed disk (for export)
- `az disk update -g resource_group -n disk_name -z new_size_in_gb` : update disk size

## Categories
disks, cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/disk
- tldr page: https://tldr.inbrowser.app/pages/common/az-disk
