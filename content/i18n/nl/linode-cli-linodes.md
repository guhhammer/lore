# linode-cli linodes

Beheer Linode instanties.

## Usage
- `linode-cli linodes list` : toon alle Linodes
- `linode-cli linodes create --type linode_type --region regio --image image_id` : maak een nieuwe Linode
- `linode-cli linodes view linode_id` : bekijk details van een specifieke Linode
- `linode-cli linodes update linode_id --label nieuw_label` : werk de instellingen bij voor een Linode
- `linode-cli linodes delete linode_id` : verwijder een Linode
- `linode-cli linodes boot|reboot|shutdown linode_id` : voer een stroombeheeroperatie uit op een Linode
- `linode-cli linodes backups-list linode_id` : toon alle beschikbare backups van een Linode
- `linode-cli linodes backups-restore linode_id --backup-id backup_id` : zet een backup terug naar een Linode
