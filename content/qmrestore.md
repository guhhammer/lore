# qmrestore

Restore QemuServer `vzdump` backups.

## Usage
- `qmrestore path/to/vzdump-qemu-100.vma.lzo 100` : restore virtual machine from given backup file on the original storage
- `qmrestore path/to/vzdump-qemu-100.vma.lzo 100 --force true` : overwrite existing virtual machine from a given backup file on the original storage
- `qmrestore path/to/vzdump-qemu-100.vma.lzo 100 --storage local` : restore the virtual machine from a given backup file on specific storage
- `qmrestore path/to/vzdump-qemu-100.vma.lzo 100 --live-restore true` : start virtual machine immediately from the backup while restoring in the background (only on Proxmox Backup Server)

## Categories
files

## Links
- More information: https://pve.proxmox.com/pve-docs/qmrestore.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/qmrestore
