# compress a folder

Pack a folder into one compressed file, look inside it, and unpack it
again.

## Steps
- `tar -czf folder.tar.gz folder/` : pack and compress with gzip, fast
- `tar -cJf folder.tar.xz folder/` : with xz: smaller, slower
- `zip -r folder.zip folder/` : a zip, for people on Windows
- `tar -tzf folder.tar.gz` : list what is inside, without unpacking
- `tar -xzf folder.tar.gz` : unpack it here
- `unzip folder.zip` : unpack a zip

## Notes
- `lore explain "tar -czf folder.tar.gz folder/"` says what each letter does.
- GNU tar also does zstd, fast and small: `tar --zstd -cf folder.tar.zst folder/`.

## See also
- tar
- zip
- unzip
- gzip
- xz

## Categories
archives files
