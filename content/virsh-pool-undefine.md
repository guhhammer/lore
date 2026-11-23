# virsh pool-undefine

Delete the configuration file in `/etc/libvirt/storage` for a stopped virtual machine storage pool.

## Usage
- `virsh pool-undefine --pool name|uuid` : delete the configuration for the storage pool specified name or UUID (determine using `virsh pool-list`)

## See also
- virsh pool-destroy

## Categories
files

## Links
- More information: https://manned.org/virsh
- tldr page: https://tldr.inbrowser.app/pages/common/virsh-pool-undefine
