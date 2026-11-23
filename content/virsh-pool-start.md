# virsh pool-start

Start a previously configured but inactive virtual machine storage pool.

## Usage
- `virsh pool-start --pool name|uuid --build` : start the storage pool specified by name or UUID (determine using `virsh pool-list`) and create the underlying storage system if it doesn't exist

## See also
- virsh pool-define-as
- virsh pool-destroy

## Links
- More information: https://manned.org/virsh
- tldr page: https://tldr.inbrowser.app/pages/common/virsh-pool-start
