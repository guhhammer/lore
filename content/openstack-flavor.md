# openstack flavor

Manage OpenStack instance flavors (virtual hardware templates).

## Usage
- `openstack flavor list` : list all flavors
- `openstack flavor show flavor_id_or_name` : show details of a flavor
- `openstack flavor create flavor_name --vcpus 2 --ram 4096 --disk 20` : create a new flavor with 2 vCPUs, 4GB RAM, and 20GB disk
- `openstack flavor delete flavor_id_or_name` : delete a flavor
- `openstack flavor create flavor_name --vcpus 4 --ram 8192 --disk 40 --ephemeral 10 --swap 512` : create a flavor with 10GB ephemeral disk and 512MB swap space

## Categories
disks

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/openstack-flavor
