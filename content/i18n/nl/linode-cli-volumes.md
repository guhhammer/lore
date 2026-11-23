# linode-cli volumes

Beheer Linode Volumes.

## Usage
- `linode-cli volumes list` : toon alle huidige Volumes
- `linode-cli volumes create --label volume_label --size size_in_GB --linode-id linode_id` : maak een nieuw Volume en koppel het aan een specifieke Linode
- `linode-cli volumes attach volume_id --linode-id linode_id` : koppel een Volume aan een specifieke Linode
- `linode-cli volumes detach volume_id` : koppel een Volume los van een Linode
- `linode-cli volumes resize volume_id --size new_size_in_GB` : vergroot een Volume (Let op: de grootte kan alleen toenemen)
- `linode-cli volumes delete volume_id` : verwijder een Volume
