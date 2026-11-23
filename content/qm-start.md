# qm start

Start a virtual machine on QEMU/KVM Virtual Machine Manager.

## Usage
- `qm start 100` : start a specific virtual machine
- `qm start 100 --machine q35` : specify the QEMU machine type (i.e. the CPU to emulate)
- `qm start 100 --timeout 60` : start a specific virtual machine with a timeout in 60 seconds

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_start
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-start
