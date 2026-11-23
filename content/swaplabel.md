# swaplabel

Print or change the label or UUID of a swap area. Note: `path/to/file` can either point to a regular file or a swap partition.

## Usage
- `swaplabel path/to/file` : display the current label and UUID of a swap area
- `swaplabel -L new_label path/to/file` : set the label of a swap area
- `swaplabel -U new_uuid path/to/file` : set the UUID of a swap area (you can generate a UUID using `uuidgen`)

## Categories
files, disks, documents

## Links
- More information: https://manned.org/swaplabel
- tldr page: https://tldr.inbrowser.app/pages/linux/swaplabel
