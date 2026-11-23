# exo compute instance

Manage Exoscale Compute instances.

## Usage
- `exo compute instance create --disk-size 10 instance_name -z zone --template 'Linux Debian 12 (Bookworm) 64-bit'` : create a Debian based Compute instance with a disk size of 10GB
- `exo compute instance ssh instance_name|id` : log into a Compute instance via SSH
- `exo compute instance list` : list all of the Compute instances
- `exo compute instance security-group add instance_name|id security_group_name|id` : add an instance to a Security Group
- `exo compute instance scale instance_name|id instance_type` : scale the size of a Compute instance
- `exo compute instance snapshot create instance_name|id` : create a snapshot of a Compute instance
- `exo compute instance snapshot revert snapshot_id instance_name|id` : revert a Compute instance to a snapshot (the data written after the snapshot has been created will be lost)
- `exo compute instance resize-disk instance_name|id 20` : resize the disk size of a Compute instance to 20GB

## Categories
disks, network

## Links
- More information: https://community.exoscale.com/product/compute/instances/
- tldr page: https://tldr.inbrowser.app/pages/common/exo-compute-instance
