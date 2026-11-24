# lxc network

Manage networks for LXD containers.

## Usage
- `lxc network list` : list all available networks
- `lxc network show network_name` : show the configuration of a specific network
- `lxc network attach network_name container_name` : add a running instance to a specific network
- `lxc network create network_name` : create a new managed network
- `lxc network set network_name bridge.external_interfaces eth0` : set a bridge interface of a specific network
- `lxc network set network_name ipv4.nat false` : disable NAT for a specific network

## Categories
network, containers

## Links
- More information: https://canonical.com/lxd/docs/latest/reference/manpages/lxc/network/
- tldr page: https://tldr.inbrowser.app/pages/linux/lxc-network
- Wikipedia: https://en.wikipedia.org/wiki/LXC
