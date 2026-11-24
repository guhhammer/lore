# unmount a busy drive

"target is busy": something still uses the drive. Find it, stop it, then unmount.

## Steps
- `sudo lsof +f -- /mnt/point` : the open files on it, and their programs
- `sudo fuser -vm /mnt/point` : the programs using it
- `cd ~` : your own shell may be the one inside it
- `sudo umount /mnt/point` : unmount it
- `sudo umount -l /mnt/point` : lazy: detach it now, finish when nothing uses it

## See also
- lsof
- fuser
- umount

## Categories
disks
