# virt-clone

Clone a libvirt virtual machine.

## Usage
- `virt-clone -o vm_name --auto-clone` : clone a virtual machine and automatically generate a new name, storage path, and MAC address
- `virt-clone -o vm_name -n new_vm_name -f path/to/new_storage -m ff:ff:ff:ff:ff:ff|RANDOM` : clone a virtual machine and specify the new name, storage path, and MAC address

## Categories
files

## Links
- More information: https://manned.org/virt-clone
- tldr page: https://tldr.inbrowser.app/pages/linux/virt-clone
