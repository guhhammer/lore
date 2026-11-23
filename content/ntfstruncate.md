# ntfstruncate

Truncate a file on an NTFS volume.

## Usage
- `ntfstruncate [options] device file [attr-type [attr-name]] new-length` : the general form, as the manual gives it
- `ntfstruncate -h` : show a list of options with a brief description of each one
- `ntfstruncate -l` : display licensing information
- `ntfstruncate -n` : simulate the truncation without actually write to device
- `ntfstruncate -q` : suppress some debug/warning/error messages

## Categories
files, disks

## Links
- Documentation: https://github.com/tuxera/ntfs-3g/wiki/
