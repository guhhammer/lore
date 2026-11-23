# ntfscp

Copy file to an NTFS volume.

## Usage
- `ntfscp [options] device source_file destination` : the general form, as the manual gives it
- `ntfscp -a` : write to this attribute
- `ntfscp -i` : treat destination as inode number
- `ntfscp -m` : minimize fragmentation when allocating space to the attribute
- `ntfscp -N` : write to attribute with this name

## Categories
files, disks

## Links
- Documentation: https://github.com/tuxera/ntfs-3g
