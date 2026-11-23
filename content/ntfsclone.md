# ntfsclone

Efficiently clone, image, restore or rescue an NTFS.

## Usage
- `ntfsclone [OPTIONS] SOURCE` : the general form, as the manual gives it
- `ntfsclone --save-image [OPTIONS] SOURCE` : another form of the same command
- `ntfsclone --restore-image [OPTIONS] SOURCE` : another form of the same command
- `ntfsclone --metadata [OPTIONS] SOURCE` : another form of the same command
- `ntfsclone -s` : save to the special image format
- `ntfsclone -r` : restore from the special image format specified by SOURCE argument
- `ntfsclone --ignore-fs-check` : ignore the result of the filesystem consistency check

## Categories
containers, media

## Links
- Documentation: https://github.com/tuxera/ntfs-3g/wiki/
