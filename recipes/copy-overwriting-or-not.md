# copy, overwriting or not

Copy without being asked each time, or without ever replacing a file that is there.

## Steps
- `alias cp` : see whether cp is an alias for cp -i, which asks
- `\cp -f src dest` : copy without the alias, replacing dest
- `cp -n src dest` : never replace a file that is there
- `cp -i src dest` : ask before replacing
- `rsync -a --ignore-existing src/ dest/` : copy a folder, only the files dest does not have
- `cp -u src dest` : replace only when src is newer (GNU)

## See also
- cp
- rsync
- alias

## Categories
files
