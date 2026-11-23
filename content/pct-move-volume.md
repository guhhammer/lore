# pct move-volume

Move a volume to a different storage or to a different container.

## Usage
- `pct mov 100 rootfs storage_id` : move the root filesystem of a container to a different storage
- `pct mov 100 rootfs storage_id --delete` : delete the filesystem association to the old volume once the move is complete

## Categories
disks, files, containers

## Links
- More information: https://pve.proxmox.com/pve-docs/pct.1.html#cli_pct_move-volume
- tldr page: https://tldr.inbrowser.app/pages/linux/pct-move-volume
