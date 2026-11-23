# nova

The OpenStack project that provides a way to provision compute instances.

## Usage
- `nova list` : list VMs on current tenant
- `nova list --all-tenants` : list VMs of all tenants (admin user only)
- `nova boot --nic net-id=net_id --image image_id --flavor flavor --availability-zone nova:host_name vm_name` : boot a VM on a specific host
- `nova start server` : start a server
- `nova stop server` : stop a server
- `nova interface-attach --net-id net_id server` : attach a network interface to a specific VM

## Categories
network, development

## Links
- More information: https://docs.openstack.org/nova/latest/
- tldr page: https://tldr.inbrowser.app/pages/linux/nova
