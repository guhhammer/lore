# fsck.exfat

Check an exFAT filesystem.

## Usage
- `fsck.exfat [ -a ] [ -n ] [ -r ] [ -v ] [ -y ] [ -b ] [ -v ] device` : the general form, as the manual gives it
- `fsck.exfat -V` : another form of the same command
- `fsck.exfat -n` : check the filesystem but do not attempt to repair the filesystem
- `fsck.exfat -r` : repair the filesystem interactively
- `fsck.exfat -s` : create  files in /LOST+FOUND for orphan clusters
- `fsck.exfat -V` : prints the version number and exits

## Categories
disks
