# virsh pool-build

Build the underlying storage system for a virtual machine storage pool as defined in its configuration file in `/etc/libvirt/storage`.

## Usage
- `virsh pool-build --pool name|uuid` : build the storage pool specified by name or UUID (determine using `virsh pool-list`)

## See also
- virsh pool-define-as
- virsh pool-start

## Categories
development, files

## Links
- More information: https://manned.org/virsh
- tldr page: https://tldr.inbrowser.app/pages/common/virsh-pool-build
