# virsh pool-define-as

Create a configuration file in `/etc/libvirt/storage` for a persistent virtual machine storage pool from the provided arguments.

## Usage
- `virsh pool-define-as --name pool_name --type dir --target /var/vms` : create the configuration file for a storage pool called pool_name using `/var/vms` as the underlying storage system

## See also
- virsh pool-build
- virsh pool-start

## Categories
files

## Links
- More information: https://manned.org/virsh
- tldr page: https://tldr.inbrowser.app/pages/common/virsh-pool-define-as
