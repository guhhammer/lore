# qm create

Create or restore a virtual machine on QEMU/KVM Virtual Machine Manager.

## Usage
- `qm cr 100` : create a virtual machine with access to 512MiB of memory and 1 CPU core
- `qm cr 100 --name vm_name --start` : give the virtual machine a name and automatically start the machine after creation
- `qm cr 100 --memory 8192 --cores 4` : give a virtual machine specified amount of memory and CPUs
- `qm cr 100 --ostype win10` : specify the type of operating system on the machine
- `qm cr 100 --archive path/to/backup_file.tar --force 1` : replace an existing machine (requires archiving it)
- `qm cr 100 --hookscript path/to/script.pl` : specify a script that is executed automatically depending on the state of the virtual machine
- `qm cr 100 --cdrom local:iso/install-media.iso` : specify the install media
- `qm cr 100 --net0 virtio,bridge=vmbr0` : create a VM that bridges itself to the host network

## Categories
network, monitoring

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_create
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-create
