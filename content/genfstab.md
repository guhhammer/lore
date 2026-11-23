# genfstab

Generate output suitable for addition to the `/etc/fstab` file.

## Usage
- `genfstab -U /mnt >>/mnt/etc/fstab` : generate the `/etc/fstab` file using volume UUIDs during an Arch Linux installation (requires root permissions)
- `genfstab -L path/to/mount_point` : display fstab-compatible output based on volume labels
- `genfstab -U path/to/mount_point` : display fstab-compatible output based on volume UUIDs
- `genfstab -t LABEL|UUID|PARTLABEL|PARTUUID` : display fstab-compatible output based on the specified identifier
- `genfstab -U path/to/mount_point | sudo tee -a /etc/fstab` : append a volume into the `/etc/fstab` file to mount it automatically

## Categories
files, disks, desktop

## Links
- More information: https://manned.org/genfstab
- tldr page: https://tldr.inbrowser.app/pages/linux/genfstab
