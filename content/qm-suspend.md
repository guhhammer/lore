# qm suspend

Suspend a virtual machine (VM) in the Proxmox Virtual Environment (PVE). Use `--skiplock` and `--skiplockstorage` flags with caution, as they may lead to data corruption in certain situations.

## Usage
- `qm su 100 integer` : suspend a virtual machine by ID
- `qm su 100 integer --skiplock` : skip the lock check when suspending the VM
- `qm su 100 integer --skiplockstorage` : skip the lock check for storage when suspending the VM

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_suspend
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-suspend
