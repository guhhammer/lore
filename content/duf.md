# duf

Disk Usage/Free Utility.

## Usage
- `duf` : list accessible devices
- `duf --all` : list everything (such as pseudo, duplicate, or inaccessible filesystems)
- `duf path/to/directory1 path/to/directory2 ...` : only show specified devices or mount points
- `duf --sort size|used|avail|usage` : sort the output by a specified criteria
- `duf --only-fs|hide-fs tmpfs|vfat|ext4|xfs` : show or hide specific filesystems
- `duf --sort mountpoint|size|used|avail|usage|inodes|inodes_used|inodes_avail|inodes_usage|type|filesystem` : sort the output by key
- `duf --theme dark|light` : change the theme (if `duf` fails to use the right theme)

## See also
- ncdu
- df

## Categories
monitoring, disks

## Links
- More information: https://github.com/muesli/duf#usage
- tldr page: https://tldr.inbrowser.app/pages/common/duf
