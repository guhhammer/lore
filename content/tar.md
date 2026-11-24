# tar

Packs files and folders into one archive file, usually compressed
(.tar.gz, .tar.xz), and unpacks them again.

## Usage
- `tar -xf archive.tar.gz` : unpack an archive (the compression is detected)
- `tar -tf archive.tar.gz` : list what is inside without unpacking
- `tar -czf backup.tar.gz folder/` : pack a folder, gzip-compressed
- `tar -cJf backup.tar.xz folder/` : the same, xz (smaller, slower)
- `tar -xf archive.tar.gz -C /tmp` : unpack into another folder

## Categories
archives

## Links
- GNU manual: https://www.gnu.org/software/tar/manual/tar.html
- Wikipedia: https://en.wikipedia.org/wiki/Tar_(computing)
