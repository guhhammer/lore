# pvenode config

Get and set node configuration.

## Usage
- `pvenode co g` : display the whole configuration located in `/etc/pve/nodes/node_name/config`
- `pvenode co g --property property_name` : display a specific property
- `pvenode co s --description description_text` : set the node description shown in the "Notes" tab
- `pvenode co s --location latitude=latitude,longitude=longitude` : set the location information of a node
- `pvenode co s --delete property_name` : delete a setting

## Links
- More information: https://pve.proxmox.com/pve-docs/pvenode.1.html#cli_pvenode_config_get
- tldr page: https://tldr.inbrowser.app/pages/linux/pvenode-config
