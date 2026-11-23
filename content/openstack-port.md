# openstack port

Manage OpenStack network ports (virtual network interfaces).

## Usage
- `openstack port list` : list all ports
- `openstack port show port_id_or_name` : show detailed information about a specific port
- `openstack port create --network network_id_or_name port_name` : create a port on a specific network
- `openstack port create --network network_id --fixed-ip subnet=subnet_id,ip-address=192.168.1.50 port_name` : create a port and assign it a fixed IP `192.168.1.50`
- `openstack port delete port_id_or_name` : delete a port

## Categories
network

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/openstack-port
