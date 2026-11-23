# lvcreate

Create a logical volume in an existing volume group. A volume group is a collection of logical and physical volumes.

## Usage
- `sudo lvcreate -L 10G vg1` : create a logical volume of 10 gigabytes in the volume group `vg1`
- `sudo lvcreate -L 1500 -n mylv vg1` : create a 1500 megabyte linear logical volume named `mylv` in the volume group `vg1`
- `sudo lvcreate -l 60%VG -n mylv vg1` : create a logical volume called `mylv` that uses 60% of the total space in volume group `vg1`
- `sudo lvcreate -l 100%FREE -n mylv vg1` : create a logical volume called `mylv` that uses all the unallocated space in the volume group `vg1`

## See also
- lvm

## Categories
disks, users

## Links
- More information: https://manned.org/lvcreate
- tldr page: https://tldr.inbrowser.app/pages/linux/lvcreate
