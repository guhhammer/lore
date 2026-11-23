# pvck

Check and repair LVM metadata on physical volumes.

## Usage
- `sudo pvck /dev/sdXN --dump headers` : print LVM on-disk headers and structures (label, PV header, MDA header, metadata area)
- `sudo pvck /dev/sdXN --dump metadata` : print the current VG metadata text
- `sudo pvck /dev/sdXN --dump metadata_all` : list all metadata versions found in the metadata area
- `sudo pvck /dev/sdXN --dump metadata_search -f path/to/metadata.txt` : search common locations for metadata when headers may be damaged, and save it to a file
- `sudo pvck /dev/sdXN --dump metadata --settings "mda_num=2"` : select the second metadata area (mda2) when printing metadata
- `sudo pvck /dev/sdXN --repair -f path/to/metadata_file` : repair headers and metadata using a metadata input file (use with care)
- `sudo pvck /dev/sdXN --repairtype pv_header` : repair only the PV header and label header

## Categories
users, disks, files

## Links
- More information: https://manned.org/pvck
- tldr page: https://tldr.inbrowser.app/pages/linux/pvck
