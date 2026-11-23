# qm

QEMU/KVM Virtual Machine Manager. Some subcommands such as `list`, `start`, `stop`, `clone`, etc. have their own usage documentation.

## Usage
- `qm list` : list all virtual machines
- `qm cr 100 --scsi0 local-lvm:4 --net0 e1000 --cdrom local:iso/proxmox-mailgateway_2.1.iso` : create a virtual machine with a 4 GB SCSI disk on the `local-lvm` storage and an ID of 100 using an ISO file uploaded on the `local` storage
- `qm co 100` : show the configuration of a virtual machine, specifying its ID
- `qm start 100` : start a specific virtual machine
- `qm shu 100 && qm w 100` : send a shutdown request, then wait until the virtual machine is stopped
- `qm des 100 --purge` : destroy a virtual machine and remove all related resources

## Categories
disks, documents, monitoring

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/qm
