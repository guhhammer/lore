# vzdump

Backup Utility for virtual machines and containers.

## Usage
- `vzdump vm_id` : dump a guest virtual machine into the default dump directory (usually `/var/lib/vz/dump/`), excluding snapshots
- `vzdump 101 102 103` : back up the guest virtual machines with the IDs 101, 102, and 103
- `vzdump vm_id --mode suspend|snapshot` : dump a guest virtual machine using a specific mode
- `vzdump --all --mode suspend --mailto root --mailto admin` : back up all guest systems and send a notification email to the root and admin users
- `vzdump vm_id --dumpdir path/to/directory --mode snapshot` : use snapshot mode (no downtime required) and a non-default dump directory
- `vzdump --mode suspend --exclude 101, 102` : back up all guest virtual machines excluding the IDs 101 and 102

## Categories
files, containers

## Links
- More information: https://pve.proxmox.com/pve-docs/vzdump.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/vzdump
