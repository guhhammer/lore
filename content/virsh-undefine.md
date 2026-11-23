# virsh undefine

Delete a virtual machine.

## Usage
- `virsh undefine --domain vm_name` : delete only the virtual machine configuration file
- `virsh undefine --domain vm_name --remove-all-storage` : delete the configuration file and all associated storage volumes
- `virsh undefine --domain vm_name --storage sda,sdb,path/to/source,...` : delete the configuration file and the specified storage volumes using the target name or the source name (as obtained from the `virsh domblklist` command)

## Categories
files

## Links
- More information: https://manned.org/virsh
- tldr page: https://tldr.inbrowser.app/pages/common/virsh-undefine
