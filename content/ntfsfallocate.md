# ntfsfallocate

Preallocate space to a file on an NTFS volume.

## Usage
- `ntfsfallocate [options] -l length device file [attr-type [attr-name]]` : the general form, as the manual gives it
- `ntfsfallocate -h` : show a list of options with a brief description of each one
- `ntfsfallocate -N` : simulate the allocation without actually write to device
- `ntfsfallocate -o` : specify the offset in the file where preallocation starts
- `ntfsfallocate -q` : suppress some debug/warning/error messages

## Categories
files, disks

## Links
- Documentation: https://github.com/tuxera/ntfs-3g/wiki/
