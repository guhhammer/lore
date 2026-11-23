# openstack subnet

Manage OpenStack subnets (IP address blocks within a network).

## Usage
- `openstack subnet list` : list all subnets
- `openstack subnet show subnet_id_or_name` : show details of a specific subnet
- `openstack subnet list --network network_id_or_name` : list subnets associated with a network
- `openstack subnet create --network network_id_or_name --subnet-range 192.168.0.0/24 subnet_name` : create a subnet with subnet range `192.168.0.0/24` in a given network
- `openstack subnet delete subnet_id_or_name` : delete a subnet
- `openstack subnet set --dns-nameserver 8.8.8.8 --name new_subnet_name subnet_id` : update a subnet with DNS `8.8.8.8` and set a new name

## Categories
network

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/openstack-subnet
