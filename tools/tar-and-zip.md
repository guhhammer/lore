# tar and zip

tar bundles files into one archive and gzip, xz or zstd shrink it; zip does both for people on Windows and macOS. These are the commands for making, listing and unpacking them.

## Tasks
### tar: make
- `tar -czf archive.tar.gz DIR` : pack a folder with gzip
- `tar -cJf archive.tar.xz DIR` : pack with xz (smaller, slower)
- `tar --zstd -cf archive.tar.zst DIR` : pack with zstd (fast and small)
- `tar -czf archive.tar.gz --exclude='*.log' DIR` : skip some files
- `tar -czf archive.tar.gz -C /path DIR` : pack from another folder without the full path
### tar: look and unpack
- `tar -tf archive.tar.gz` : list the contents
- `tar -xf archive.tar.gz` : unpack here, any compression
- `tar -xf archive.tar.gz -C DEST` : unpack into a folder
- `tar -xf archive.tar.gz PATH/IN/ARCHIVE` : unpack just one file
- `tar -xzvf archive.tar.gz` : unpack and print each name
- `tar -xf archive.tar.gz --strip-components=1` : drop the top folder when unpacking
### zip
- `zip -r archive.zip DIR` : pack a folder
- `zip -r -e archive.zip DIR` : pack with a password
- `zip archive.zip FILE1 FILE2` : pack some files
- `zip -r archive.zip DIR -x '*.git*'` : skip files
- `unzip archive.zip` : unpack here
- `unzip archive.zip -d DEST` : unpack into a folder
- `unzip -l archive.zip` : list the contents
- `unzip -o archive.zip` : unpack and overwrite without asking (replaces files)
### Single files and others
- `gzip FILE` : compress a file, replacing it with FILE.gz
- `gunzip FILE.gz` : the reverse
- `gzip -k FILE` : keep the original
- `zcat FILE.gz | less` : read a compressed file without unpacking it
- `xz -d FILE.xz` : unpack an xz file
- `7z x archive.7z` : unpack a 7z archive (package p7zip)
- `unrar x archive.rar` : unpack a rar archive

## Notes
- `c` creates, `x` extracts, `t` lists, `f` names the archive: `f` must come right before the file name.
- tar picks the right decompressor when you extract; for creating you choose the flag.
- Archives from strangers can hold paths like `../`: list with `-t` before unpacking.
- `zip` does not keep Linux permissions or owners well; use tar for backups.

## See also
- rsync
- gzip
- xz
- 7z

## Links
- GNU tar: https://www.gnu.org/software/tar/manual/tar.html
- zip: https://infozip.sourceforge.net/Zip.html

## Categories
archives files
