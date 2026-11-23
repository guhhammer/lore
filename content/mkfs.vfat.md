# mkfs.vfat

This command is an alias of `mkfs.fat`.

## Usage
- `sudo mkfs.fat /dev/sdXY` : create a fat filesystem inside partition `Y` on device `X`
- `sudo mkfs.fat -n volume_name /dev/sdXY` : create filesystem with a volume-name
- `sudo mkfs.fat -i volume_id /dev/sdXY` : create filesystem with a volume-id
- `sudo mkfs.fat -f 4 /dev/sdXY` : use 4 instead of 2 file allocation tables
- `sudo mkfs.fat -F 12|16|32 /dev/sdXY` : specify filesystem type

## Categories
disks, users, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/mkfs.vfat
