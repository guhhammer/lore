# pvscan

List all physical volumes and manage their online status.

## Usage
- `sudo pvscan` : list all physical volumes
- `sudo pvscan --cache --listvg /dev/sdX` : show the volume group that uses a specific physical volume
- `sudo pvscan --cache --listlvs /dev/sdX` : show logical volumes that use a specific physical volume
- `sudo pvscan --reportformat json` : display detailed information in JSON format

## Categories
users, disks

## Links
- More information: https://manned.org/pvscan
- tldr page: https://tldr.inbrowser.app/pages/linux/pvscan
