# stratis

Manage local storage pools and volumes using the Stratis storage manager. Stratis volumes use the XFS filesystem and require the `stratisd` service.

## Usage
- `sudo systemctl start stratisd` : start the Stratis service (must be active before managing pools or volumes)
- `sudo stratis pool create pool_name /dev/sdX /dev/sdY ...` : create a storage pool from one or more devices
- `sudo stratis filesystem create pool_name volume_name` : create a filesystem (volume) in a pool
- `sudo stratis filesystem list` : list all Stratis filesystems
- `sudo mkfs.xfs /dev/stratis/pool_name/volume_name && sudo mount /dev/stratis/pool_name/volume_name /mnt/target` : format and mount a Stratis volume manually
- `echo /dev/stratis/pool_name/volume_name /mnt/target xfs defaults,x-systemd.requires=stratisd.service 0 0 | sudo tee -a /etc/fstab` : add a volume to `/etc/fstab` for mounting at boot
- `sudo stratis pool add-data pool_name /dev/sdZ` : extend an existing pool by adding a new device
- `sudo stratis filesystem destroy pool_name volume_name` : delete a volume

## Categories
disks, users, system

## Links
- More information: https://stratis-storage.github.io/howto/
- tldr page: https://tldr.inbrowser.app/pages/linux/stratis
