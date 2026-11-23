# qm migrate

Migrate a virtual machine. Used to create a new migration task.

## Usage
- `qm mi 100 target` : migrate a specific virtual machine
- `qm mi 100 target --bwlimit 10` : override the current I/O bandwidth limit with 10 KiB/s
- `qm mi 100 target --force true` : allow migration of virtual machines using local devices (root only)
- `qm mi 100 target --online true` : use online/live migration if a virtual machine is running
- `qm mi 100 target --with-local-disks true` : enable live storage migration for local disks

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_migrate
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-migrate
