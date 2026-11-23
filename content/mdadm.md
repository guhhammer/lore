# mdadm

RAID management utility.

## Usage
- `sudo mdadm --create /dev/md/MyRAID --level raid_level --raid-devices number_of_disks /dev/sdXN` : create array
- `sudo mdadm --stop /dev/md0` : stop array
- `sudo mdadm --fail /dev/md0 /dev/sdXN` : mark disk as failed
- `sudo mdadm --remove /dev/md0 /dev/sdXN` : remove disk
- `sudo mdadm --assemble /dev/md0 /dev/sdXN` : add disk to array
- `sudo mdadm --detail /dev/md0` : show RAID info
- `sudo mdadm --zero-superblock /dev/sdXN` : reset disk by deleting RAID metadata

## Categories
danger, disks, users

## Links
- More information: https://manned.org/mdadm
- tldr page: https://tldr.inbrowser.app/pages/linux/mdadm
