# copy a folder

Copy a folder with everything inside it, keeping its dates and permissions if needed.

## Steps
- `cp -r src dest` : copy src and everything in it to dest
- `cp -a src dest` : the same, keeping permissions, dates and links
- `rsync -a src/ dest/` : copy what is inside src into dest; again later copies only what changed
- `rsync -a --progress src/ dest/` : with progress, for big copies

## Notes
- With rsync the slash matters: `src/` copies what is inside, `src` makes dest/src.

## See also
- cp
- rsync

## Categories
files
