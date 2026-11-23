# qm cleanup

Clean up resources on QEMU/KVM Virtual Machine Manager like tap devices, VGPUs, etc. Called after a VM shuts down, crashes, etc.

## Usage
- `qm cle 100 true|false true|false` : clean up resources
- `qm cle 100 false true|false` : indicate to the system that the VM was not shut down cleanly
- `qm cle 100 true|false true` : indicate to the system that the shutdown was requested by the guest

## Categories
hardware

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_cleanup
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-cleanup
