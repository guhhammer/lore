# openstack volume

Manage OpenStack volumes. OpenStack Block Storage service, aka OpenStack Cinder, provides volumes to Nova vm's, Ironic bare-metal hosts, containers, and others.

## Usage
- `openstack volume list --all-projects` : list volumes
- `openstack volume show volume_id` : show volume details
- `openstack volume create --size size_in_GB --image image_id --snapshot snapshot_id --bootable|--non-bootable volume_name` : create new volume
- `openstack volume delete volume_id1 volume_id2 ...` : delete volumes(s)
- `openstack volume migrate --host host_hostname instance_id` : migrate volume to a new host
- `openstack volume set --name volume_new_name --size volume_new_size --attached|--detached --bootable|--non-bootable volume_id` : set volume properties

## Categories
disks, containers, network

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/openstack-volume
