# qm destroy

Destroy a virtual machine in QEMU/KVM Virtual Machine Manager.

## Usage
- `qm des 100` : destroy a specific virtual machine
- `qm des 100 --destroy-unreferenced-disks` : destroy all disks that are not explicitly referenced in a specific virtual machine's configuration
- `qm des 100 --purge` : destroy a virtual machine and remove from all locations (inventory, backup jobs, high availability managers, etc.)
- `sudo qm des 100 --skiplock` : destroy a specific virtual machine ignoring locks and forcing destroy

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_destroy
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-destroy
