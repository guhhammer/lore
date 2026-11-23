# virsh snapshot-delete

Delete a snapshot.

## Usage
- `sudo virsh snapshot-delete "vm_name" "snapshot_name"` : delete a snapshot and merge it to its children
- `sudo virsh snapshot-delete "vm_name" "snapshot_name" --metadata` : delete only the metadata, leaving the snapshot contents in place
- `virsh snapshot-delete --help` : display help

## Links
- More information: https://manned.org/virsh
- tldr page: https://tldr.inbrowser.app/pages/common/virsh-snapshot-delete
