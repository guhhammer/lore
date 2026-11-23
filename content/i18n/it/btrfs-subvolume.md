# btrfs subvolume

Gestisce sottovolumi e snapshot btrfs.

## Usage
- `sudo btrfs su c percorso/al/nuovo_sottovolume` : crea un nuovo sottovolume vuoto
- `sudo btrfs su l percorso/al/filesystem_btrfs` : elenca tutti i sottovolumi e snapshot nel filesystem specificato
- `sudo btrfs su d percorso/al/sottovolume` : elimina un sottovolume
- `sudo btrfs su sn -r percorso/al/sottovolume_sorgente percorso/al/target` : crea uno snapshot read-only di un sottovolume esistente
- `sudo btrfs su sn percorso/al/sottovolume_sorgente percorso/al/target` : crea uno snapshot read-write di un sottovolume esistente
- `sudo btrfs su sh percorso/al/sottovolume` : mostra informazioni dettagliate su un sottovolume

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-subvolume.html
