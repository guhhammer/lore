# systemctl list-automounts

List automount units currently in memory, showing mount paths and unit names.

## Usage
- `systemctl list-automounts` : list automount units currently in memory
- `systemctl list-automounts -a` : list all automount units, including inactive ones
- `systemctl list-automounts --state active|inactive|failed|...` : filter automount units by state
- `systemctl list-automounts pattern1 pattern2 ...` : filter automount units by name pattern

## See also
- systemctl list-units
- systemctl list-unit-files

## Categories
monitoring, disks

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-list-automounts
