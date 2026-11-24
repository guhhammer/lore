# check a file's permissions

See who may read, write and run a file or a folder, and why a path is refused.

## Steps
- `ls -l file.txt` : rwx for the owner, the group and everyone else, then owner and group
- `ls -ld folder` : a folder's own permissions, not its contents
- `stat -c '%a %U:%G %n' file.txt` : the permissions as a number, owner and group (GNU)
- `namei -l /path/to/file.txt` : every folder on the way: one without x refuses the whole path
- `id` : your user and groups, to compare

## Notes
- macOS: `stat -f '%Lp %Su:%Sg %N' file.txt`.

## See also
- ls
- stat
- namei
- id

## Categories
files security
