# tune.exfat

Adjust tunable filesystem parameters on an exFAT filesystem.

## Usage
- `tune.exfat -l /dev/sdXY` : print the volume label of a filesystem
- `tune.exfat -L new_label /dev/sdXY` : set the volume label of a filesystem
- `tune.exfat -u /dev/sdXY` : print the volume GUID of a filesystem
- `tune.exfat -U new_guid /dev/sdXY` : set the volume GUID of a filesystem
- `tune.exfat -i /dev/sdXY` : print the volume serial of a filesystem
- `tune.exfat -I new_serial /dev/sdXY` : set the volume serial of a filesystem

## Categories
disks, documents

## Links
- More information: https://manned.org/tune.exfat
- tldr page: https://tldr.inbrowser.app/pages/linux/tune.exfat
