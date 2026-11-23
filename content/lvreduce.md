# lvreduce

Reduce the size of a logical volume.

## Usage
- `sudo lvreduce -L 120G logical_volume` : reduce a volume's size to 120 GB
- `sudo lvreduce -L -40G -r logical_volume` : reduce a volume's size by 40 GB as well as the underlying filesystem

## See also
- lvm

## Categories
disks

## Links
- More information: https://manned.org/lvreduce
- tldr page: https://tldr.inbrowser.app/pages/linux/lvreduce
