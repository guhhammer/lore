# vboxmanage-modifyvm

Change settings for a virtual machine that is stopped.

## Usage
- `VBoxManage modifyvm uuid|vm_name --name new_name` : rename the VM
- `VBoxManage modifyvm uuid|vm_name --memory 2048 --cpus 2` : adjust memory and CPU
- `VBoxManage modifyvm uuid|vm_name --vrde on` : enable Remote Display (VRDE)
- `VBoxManage modifyvm uuid|vm_name --recording on` : enable session recording

## Links
- More information: https://www.virtualbox.org/manual/ch08.html#vboxmanage-modifyvm
- tldr page: https://tldr.inbrowser.app/pages/common/vboxmanage-modifyvm
