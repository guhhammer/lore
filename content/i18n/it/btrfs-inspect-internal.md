# btrfs inspect-internal

Consulta informazioni interne di un filesystem btrfs.

## Usage
- `sudo btrfs i dump-s percorso/alla/partizione` : stampa le informazioni del superblock
- `sudo btrfs i dump-s -a percorso/alla/partizione` : stampa le informazioni del superblock e tutte le sue copie
- `sudo btrfs i dump-t percorso/alla/partizione` : stampa le informazioni di metadati del filesystem
- `sudo btrfs i i n percorso/al/mount_btrfs` : stampa l'elenco dei file nell'inode `n`-esimo
- `sudo btrfs i lo logical_address percorso/al/mount_btrfs` : stampa l'elenco dei file a un dato indirizzo logico
- `sudo btrfs i t percorso/alla/partizione` : stampa le statistiche di root, extent, csum e alberi fs

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-inspect-internal.html
