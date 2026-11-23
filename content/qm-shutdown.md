# qm shutdown

Shut down a virtual machine on QEMU/KVM Virtual Machine Manager.

## Usage
- `qm shu 100` : shut down a virtual machine
- `qm shu 100 --timeout 10` : shut down a virtual machine after wait for at most 10 seconds
- `qm shu 100 --keepActive true` : shut down a virtual machine and do not deactivate storage volumes
- `qm shu 100 --skiplock true` : shut down a virtual machine and skip lock (only root can use this option)
- `qm shu 100 --forceStop true` : stop and shut down a virtual machine

## Categories
system

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_shutdown
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-shutdown
