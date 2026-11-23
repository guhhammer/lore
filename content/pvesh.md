# pvesh

Interface with the Proxmox VE API.

## Usage
- `pvesh g /nodes` : list available nodes
- `pvesh g /nodes/node_name/lxc|qemu` : display detailed information about containers or virtual machines
- `pvesh l /` : discover API paths
- `pvesh u /pools` : display API path usage instructions
- `pvesh create /storage --storage storage_id --vgname volume_group --type lvmthin --thinpool thinpool_name --content content_type1,content_type2,...` : add a new thinpool to Proxmox

## Categories
network

## Links
- More information: https://pve.proxmox.com/pve-docs/pvesh.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pvesh
