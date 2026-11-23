# virsh connect

Connect to a virtual machine hypervisor.

## Usage
- `virsh connect` : connect to the default hypervisor
- `virsh connect qemu:///system` : connect as root to the local QEMU/KVM hypervisor
- `virsh connect qemu:///session` : launch a new instance of the hypervisor and connect to it as the local user
- `virsh connect qemu+ssh://user_name@host_name/system` : connect as root to a remote hypervisor using SSH

## Categories
network

## Links
- More information: https://manned.org/virsh
- tldr page: https://tldr.inbrowser.app/pages/common/virsh-connect
