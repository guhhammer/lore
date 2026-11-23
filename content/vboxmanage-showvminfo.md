# vboxmanage-showvminfo

Show information about registered virtual machine.

## Usage
- `VBoxManage showvminfo vm_name|uuid` : show information about a particular virtual machine
- `VBoxManage showvminfo --details vm_name|uuid` : show more detailed information about a particular virtual machine
- `VBoxManage showvminfo --machinereadable vm_name|uuid` : show information in a machine readable format
- `VBoxManage showvminfo --password-id password_id vm_name|uuid` : specify password ID if the virtual machine is encrypted
- `VBoxManage showvminfo --password path/to/password_file vm_name|uuid` : specify the password file if the virtual machine is encrypted
- `VBoxManage showvminfo --log vm_name|uuid` : show the logs of a specific virtual machine

## Categories
users

## Links
- More information: https://www.virtualbox.org/manual/ch08.html#vboxmanage-showvminfo
- tldr page: https://tldr.inbrowser.app/pages/common/vboxmanage-showvminfo
