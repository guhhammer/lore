# linode-cli linodes

Manage Linode instances.

## Usage
- `linode-cli linodes list` : list all Linodes
- `linode-cli linodes create --type linode_type --region region --image image_id` : create a new Linode
- `linode-cli linodes view linode_id` : view details of a specific Linode
- `linode-cli linodes update linode_id --label new_label` : update settings for a Linode
- `linode-cli linodes delete linode_id` : delete a Linode
- `linode-cli linodes boot|reboot|shutdown linode_id` : perform a power management operation on a Linode
- `linode-cli linodes backups-list linode_id` : list available backups for a Linode
- `linode-cli linodes backups-restore linode_id --backup-id backup_id` : restore a backup to a Linode

## Categories
system

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/linode-cli-linodes
