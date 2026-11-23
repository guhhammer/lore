# qm disk

Manage disk images.

## Usage
- `qm di resi 100 disk_name +nG` : add `n` gigabytes to a virtual disk
- `qm di m 100 scic0 destination_storage_name` : move a virtual disk
- `qm di m 100 scic0 destination_storage_name --delete` : delete the previous copy of the virtual disk
- `qm di i 100 path/to/disk storage_name --format qcow2|raw|vmdk` : import a VMDK/`.qcow2`/raw disk image using a specific storage name
- `qm di resc` : rescan all storages and update disk sizes and unused disk images
- `qm di resc --dryrun` : simulate a rescan without writing any changes to configurations
- `qm di resc --vmid 100` : specify a virtual machine by its ID
- `qm di u 100 --idlist unused0,unused1,scsi1,...` : delete a disk

## Categories
disks, containers, media

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_disk_import
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-disk
