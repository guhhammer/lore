# mmls

Display the partition layout of a volume system.

## Usage
- `mmls path/to/image_file` : display the partition table stored in an image file
- `mmls -B -i path/to/image_file` : display the partition table with an additional column for the partition size
- `mmls -i ewf image.e01 image.e02` : display the partition table in a split EWF image
- `mmls -t nested_table_type -o offset path/to/image_file` : display nested partition tables

## Categories
disks, desktop, containers

## Links
- More information: https://wiki.sleuthkit.org/Mmls/
- tldr page: https://tldr.inbrowser.app/pages/common/mmls
