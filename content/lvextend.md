# lvextend

Increase the size of a logical volume.

## Usage
- `sudo lvextend -L 120G logical_volume` : increase a volume's size to 120 GB
- `sudo lvextend -L +40G -r logical_volume` : increase a volume's size by 40 GB as well as the underlying filesystem
- `sudo lvextend -l +100%FREE logical_volume` : increase a volume's size to 100% of the free physical volume space
- `sudo lvextend -l +100%FREE -r logical_volume` : increase a volume's size to 100% of the free physical volume space and resize the underlying filesystem

## See also
- lvm

## Categories
disks, users

## Links
- More information: https://manned.org/lvextend
- tldr page: https://tldr.inbrowser.app/pages/linux/lvextend
