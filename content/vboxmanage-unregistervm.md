# vboxmanage-unregistervm

Unregister a virtual machine (VM).

## Usage
- `VBoxManage unregistervm uuid|vm_name` : unregister an existing VM
- `VBoxManage unregistervm uuid|vm_name --delete` : delete hard disk image files, all saved state files, VM logs, and XML VM machine files
- `VBoxManage unregistervm uuid|vm_name --delete-all` : delete all files from the VM

## Links
- More information: https://www.virtualbox.org/manual/ch08.html#vboxmanage-unregistervm
- tldr page: https://tldr.inbrowser.app/pages/common/vboxmanage-unregistervm
