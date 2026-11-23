# virsh pool-list

List information about virtual machine storage pools.

## Usage
- `virsh pool-list` : list the name, state, and whether autostart is enabled or disabled for active storage pools
- `virsh pool-list --all|inactive` : list information for active and inactive or just inactive storage pools
- `virsh pool-list --details` : list extended information about persistence, capacity, allocation, and available space for active storage pools
- `virsh pool-list --autostart|no-autostart` : list information for active storage pools with either autostart enabled or disabled
- `virsh pool-list --persistent|transient` : list information for active storage pools that are either persistent or transient
- `virsh pool-list --name --uuid` : list the name and UUID of active storage pools

## See also
- virsh pool-autostart
- virsh pool-define-as

## Links
- More information: https://manned.org/virsh
- tldr page: https://tldr.inbrowser.app/pages/common/virsh-pool-list
