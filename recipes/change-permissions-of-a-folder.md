# change permissions of a folder and everything in it

Give a folder, its subfolders and its files the permissions or the owner they need, without making them open to everyone.

## Steps
- `ls -ld folder` : see the folder's permissions and owner now
- `chmod -R u+rwX,go+rX folder` : you can read and write everything; others can read, and enter the folders
- `find folder -type d -exec chmod 755 {} +` : folders: rwx for you, r-x for everyone else
- `find folder -type f -exec chmod 644 {} +` : files: rw for you, read-only for everyone else
- `sudo chown -R NAME:GROUP folder` : give the folder and everything in it to another owner
- `chmod +x script.sh` : let a file be run as a program

## Notes
- `chmod -R 777` lets every user change and delete everything: it is almost never the fix.
- Capital `X` in `u+rwX` adds the x only to folders and to files that already had it.

## See also
- chmod
- chown
- find
- ls

## Categories
files security users
