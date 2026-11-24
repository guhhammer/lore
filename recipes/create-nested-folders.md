# create nested folders

Make a folder and every folder above it that is missing, and copy a file into a path that does not exist yet.

## Steps
- `mkdir -p a/b/c` : make a, a/b and a/b/c, as needed; no error if they exist
- `mkdir -p "$(dirname dest/sub/file.txt)" && cp file.txt dest/sub/file.txt` : make the folders for a copy, then copy
- `install -D file.txt dest/sub/file.txt` : the same in one step (GNU)
- `cp --parents src/sub/file.txt dest/` : copy keeping the path: dest/src/sub/file.txt (GNU)

## See also
- mkdir
- cp
- install

## Categories
files
