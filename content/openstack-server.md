# openstack server

Manage OpenStack virtual machines. OpenStack Compute service, aka OpenStack Nova, mainly hosts and manages cloud computing systems.

## Usage
- `openstack server list` : list servers
- `openstack server start instance_id1 instance_id2 ...` : start server(s)
- `openstack server stop instance_id1 instance_id2 ...` : stop server
- `openstack server create --image image_id --flavor flavor_id --network network_id --wait server_name` : create new server
- `openstack server delete instance_id1 instance_id2 ...` : delete server(s)
- `openstack server migrate --live host_hostname --shared-migration|--block-migration --wait instance_id` : migrate server to different host
- `openstack server reboot --soft|--hard --wait instance_id` : perform a soft or hard reset to the server

## Categories
system, cloud, network

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/openstack-server
