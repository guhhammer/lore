# qm clone

Create a copy of virtual machine on QEMU/KVM Virtual Machine Manager.

## Usage
- `qm clone 100 101` : copy a virtual machine
- `qm clone 100 101 --name name` : copy a virtual machine using a specific name
- `qm clone 100 101 --description description` : copy a virtual machine using a specific description
- `qm clone 100 101 --full` : copy a virtual machine creating a full copy of all disks
- `qm clone 100 101 --full --format qcow2|raw|vmdk` : copy a virtual machine using a specific format for file storage (requires `--full`)
- `qm clone 100 101 --pool pool_name` : copy a virtual machine then add it to a specific pool

## Categories
files

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_clone
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-clone
