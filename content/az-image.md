# az image

Manage custom Virtual Machine Images in Azure. Part of `azure-cli` (also known as `az`).

## Usage
- `az image list -g resource_group` : list the custom images under a resource group
- `az image create -g resource_group -n name --os-type windows|linux --source os_disk_source` : create a custom image from managed disks or snapshots
- `az image delete -n name -g resource_group` : delete a custom image
- `az image show -n name -g resource_group` : show details of a custom image
- `az image update -n name -g resource_group --set property=value` : update custom images

## Categories
containers, media, cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/image
- tldr page: https://tldr.inbrowser.app/pages/common/az-image
