# vboxmanage-export

Export virtual machines to a virtual appliance (ISO) or a cloud service.

## Usage
- `VBoxManage export --output path/to/file.ova` : specify the target OVA file
- `VBoxManage export --legacy09` : export in OVF 0.9 legacy mode
- `VBoxManage export --ovf09|ovf10|ovf20` : export in OVF (0.9|1.0|2.0) format
- `VBoxManage export --manifest` : create manifest of the exported files
- `VBoxManage export --description "vm_description"` : specify a description of the VM

## Categories
cloud, files, system

## Links
- More information: https://www.virtualbox.org/manual/ch08.html#vboxmanage-export
- tldr page: https://tldr.inbrowser.app/pages/common/vboxmanage-export
