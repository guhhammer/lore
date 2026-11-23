# dpkg-name

Rename Debian packages to full package names.

## Usage
- `dpkg-name [option...] [--] file...` : the general form, as the manual gives it
- `dpkg-name -a` : the destination filename will not have the architecture information
- `dpkg-name -k` : create a symlink, instead of moving
- `dpkg-name -s` : files  will  be moved into a subdirectory
- `dpkg-name -c` : this option can used together with the -s option

## Categories
packages, files
