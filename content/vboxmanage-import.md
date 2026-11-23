# vboxmanage-import

Import a previously exported virtual machine (VM).

## Usage
- `VBoxManage import path/to/file.ovf` : import a VM from an OVF or OVA file
- `VBoxManage import path/to/file.ovf --name vm_name` : set the name of the imported VM
- `VBoxManage import path/to/file.ovf --basefolder path/to/directory` : indicate the folder where the configuration of the imported VM will be stored
- `VBoxManage import path/to/file.ovf --register` : register the imported VM in VirtualBox
- `VBoxManage import path/to/file.ovf --dry-run` : simulate an import without actually importing
- `VBoxManage import path/to/file.ovf --ostype ostype` : set the guest OS type (one of `VBoxManage list ostypes`) for the imported VM
- `VBoxManage import path/to/file.ovf --memory 1` : set the memory (in megabytes) for the imported VM
- `VBoxManage import path/to/file.ovf --cpus 1` : set the number of CPUs for the imported VM

## Categories
files

## Links
- More information: https://www.virtualbox.org/manual/ch08.html#vboxmanage-import
- tldr page: https://tldr.inbrowser.app/pages/common/vboxmanage-import
