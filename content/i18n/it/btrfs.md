# btrfs

Un filesystem basato sul principio copy-on-write (COW) per Linux. Alcuni sottocomandi come `device` hanno la propria documentazione di utilizzo.

## Usage
- `sudo btrfs su c percorso/al/sottovolume` : crea sottovolume
- `sudo btrfs su l percorso/al/mount_point` : elenca sottovolumi
- `sudo btrfs f df percorso/al/mount_point` : mostra informazioni sull'utilizzo dello spazio
- `sudo btrfs qu e percorso/al/sottovolume` : abilita quota
- `sudo btrfs qg s percorso/al/sottovolume` : mostra quota

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs.html
