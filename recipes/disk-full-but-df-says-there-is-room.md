# disk full but df says there is room

"No space left on device" with free gigabytes: the disk is out of inodes, the slots for files. Find the folder with too many small files.

## Steps
- `df -h` : the space left
- `df -i` : the inodes left: IUse% at 100% is the problem
- `sudo du --inodes -xd 1 / | sort -n | tail` : the folders with the most files, at the top of the disk (GNU)
- `sudo find /var -xdev -type f | cut -d/ -f2-4 | sort | uniq -c | sort -n | tail` : where under /var the files are

## Notes
- The usual ones: session or cache folders, mail queues, old kernels' headers. Delete what you know is safe.

## See also
- df
- du
- find

## Categories
disks files
