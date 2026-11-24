# make a symbolic link

Make a name that points to another file or folder, see where a link points, and change or remove it.

## Steps
- `ln -s /path/to/target linkname` : make linkname point to the target
- `ls -l linkname` : see where it points
- `readlink -f linkname` : the full path it ends up at, through every link
- `ln -sfn /path/to/other linkname` : point an existing link somewhere else
- `rm linkname` : remove the link; the target stays
- `ln /path/to/file hardlink` : a hard link instead: another name for the same file, same disk only

## Notes
- Remove a link to a folder with `rm linkname`, without a slash at the end: `rm -r linkname/` would delete what is inside the target.
- A relative target is taken from where the link is, not from where you are.

## See also
- ln
- readlink
- ls

## Categories
files
