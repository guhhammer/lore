# lvresize

Change the size of a logical volume.

## Usage
- `sudo lvresize -L 120G volume_group/logical_volume` : change the size of a logical volume to 120 GB
- `sudo lvresize -L +120G -r volume_group/logical_volume` : extend the size of a logical volume as well as the underlying filesystem by 120 GB
- `sudo lvresize -l 100%FREE volume_group/logical_volume` : extend the size of a logical volume to 100% of the free physical volume space
- `sudo lvresize -L -120G -r volume_group/logical_volume` : reduce the size of a logical volume as well as the underlying filesystem by 120 GB

## See also
- lvm

## Categories
disks, users

## Links
- More information: https://manned.org/lvresize
- tldr page: https://tldr.inbrowser.app/pages/linux/lvresize
