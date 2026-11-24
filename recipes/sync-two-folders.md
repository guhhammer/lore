# sync two folders

Make one folder a copy of another, here or on a server, copying only what changed.

## Steps
- `rsync -avn --delete src/ dest/` : show what would change, without changing anything
- `rsync -av --delete src/ dest/` : make dest the same as src, deleting what src does not have
- `rsync -avz src/ NAME@host:/backup/src/` : to a server, compressed on the way
- `rsync -av --progress src/ dest/` : copy only, deleting nothing, with progress

## Notes
- --delete removes files from dest: run the -n line first.

## See also
- rsync

## Categories
files network
