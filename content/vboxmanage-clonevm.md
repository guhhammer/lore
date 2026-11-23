# vboxmanage-clonevm

Create a clone of an existing virtual machine (VM).

## Usage
- `VBoxManage clonevm vm_name` : clone the specified VM
- `VBoxManage clonevm vm_name --name new_vm_name` : specify a new name for the new VM
- `VBoxManage clonevm vm_name --basefolder path/to/directory` : indicate the folder where the new VM configuration is saved
- `VBoxManage clonevm vm_name --register` : register the cloned VM in VirtualBox

## Categories
files

## Links
- More information: https://www.virtualbox.org/manual/ch08.html#vboxmanage-clonevm
- tldr page: https://tldr.inbrowser.app/pages/common/vboxmanage-clonevm
