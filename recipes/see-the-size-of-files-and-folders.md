# see the size of files and folders

How much room each file and folder takes, here or anywhere, biggest last.

## Steps
- `du -sh *` : the size of each file and folder here
- `du -sh .[!.]* *` : the same, hidden ones included
- `du -h -d 1 | sort -h` : the folders here, sorted by size, biggest last
- `du -sh folder` : the total size of one folder
- `ls -lhS` : the files here, biggest first
- `ncdu` : browse sizes folder by folder, if ncdu is installed

## See also
- du
- ncdu
- ls
- df

## Categories
disks files
