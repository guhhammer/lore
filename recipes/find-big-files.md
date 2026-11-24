# find big files

Find the files that take the most room, on a whole disk or in a folder.

## Steps
- `find / -xdev -type f -size +500M 2>/dev/null` : every file over 500 MB on this disk
- `find ~ -type f -size +100M -exec ls -lh {} +` : files over 100 MB in your home, with their sizes
- `du -ah ~ 2>/dev/null | sort -h | tail -20` : the 20 biggest files and folders in your home
- `ls -lhS` : the files in this folder, biggest first

## Notes
- `-xdev` keeps `find` on one disk; `2>/dev/null` hides the folders you may not read.
- `+500M` means more than 500 MB; `k` and `G` work too.

## See also
- find
- du
- ncdu

## Categories
disks files
