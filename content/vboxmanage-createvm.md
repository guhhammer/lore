# vboxmanage-createvm

Create a new virtual machine.

## Usage
- `VBoxManage createvm --name vm_name` : create a new VM with default settings
- `VBoxManage createvm --name vm_name --basefolder path/to/directory` : set the base folder where the VM configuration will be stored
- `VBoxManage createvm --name vm_name --ostype ostype` : set the guest OS type (one of `VBoxManage list ostypes`) for the imported VM
- `VBoxManage createvm --name vm_name --register` : register the created VM in VirtualBox
- `VBoxManage createvm --name vm_name --group group1,group2,...` : set the VM to the specified groups
- `VBoxManage createvm --name vm_name --uuid uuid` : set the Universally Unique Identifier (UUID) of the VM
- `VBoxManage createvm --name vm_name --cipher AES-128|AES-256` : set the cipher to use for encryption

## Categories
files

## Links
- More information: https://www.virtualbox.org/manual/ch08.html#vboxmanage-createvm
- tldr page: https://tldr.inbrowser.app/pages/common/vboxmanage-createvm
