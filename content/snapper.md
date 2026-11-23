# snapper

Filesystem snapshot management tool.

## Usage
- `snapper list-configs` : list snapshot configs
- `snapper -c config create-config path/to/directory` : create snapper config
- `snapper -c config create -d "snapshot_description"` : create a snapshot with a description
- `snapper -c config list` : list snapshots for a config
- `snapper -c config delete snapshot_number` : delete a snapshot
- `snapper -c config delete snapshot1-snapshot2` : delete a range of snapshots

## Categories
disks

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/snapper
