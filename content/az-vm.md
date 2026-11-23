# az vm

Manage virtual machines in Azure. Part of `azure-cli` (also known as `az`).

## Usage
- `az vm list --output table` : display a table of available Virtual Machines
- `az vm create -g rg -n vm_name --image UbuntuLTS --admin-user azureuser --generate-ssh-keys` : create a virtual machine using the default Ubuntu image and generate SSH keys
- `az vm stop -g rg -n vm_name` : stop a Virtual Machine
- `az vm deallocate -g rg -n vm_name` : deallocate a Virtual Machine
- `az vm start -g rg -n vm_name` : start a Virtual Machine
- `az vm restart -g rg -n vm_name` : restart a Virtual Machine
- `az vm image list` : list VM images available in the Azure Marketplace

## Categories
cloud, containers, media

## Links
- More information: https://learn.microsoft.com/cli/azure/vm
- tldr page: https://tldr.inbrowser.app/pages/common/az-vm
