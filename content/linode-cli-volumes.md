# linode-cli volumes

Manage Linode Volumes.

## Usage
- `linode-cli volumes list` : list current Volumes
- `linode-cli volumes create --label volume_label --size size_in_GB --linode-id linode_id` : create a new Volume and attach it to a specific Linode
- `linode-cli volumes attach volume_id --linode-id linode_id` : attach a Volume to a specific Linode
- `linode-cli volumes detach volume_id` : detach a Volume from a Linode
- `linode-cli volumes resize volume_id --size new_size_in_GB` : resize a Volume (Note: Size can only be increased)
- `linode-cli volumes delete volume_id` : delete a Volume

## Categories
disks

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/linode-cli-volumes
