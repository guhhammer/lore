# btrfs restore

Tenta di recuperare i file da un filesystem btrfs danneggiato.

## Usage
- `sudo btrfs rest percorso/al/device/btrfs percorso/alla/directory_target` : ripristina tutti i file da un filesystem btrfs in una directory specificata
- `sudo btrfs rest -D percorso/al/device/btrfs percorso/alla/directory_target` : elenca (non scrive) i file da ripristinare da un filesystem btrfs
- `sudo btrfs rest --path-regex regex -c percorso/al/device/btrfs percorso/alla/directory_target` : ripristina file che corrispondono a un dato `regex` (case-insensitive) da un filesystem btrfs (tutte le directory genitore dei file target devono corrispondere)
- `sudo btrfs rest -t bytenr percorso/al/device/btrfs percorso/alla/directory_target` : ripristina file da un filesystem btrfs usando un albero root specifico tree `bytenr` (vedi `btrfs-find-root`)
- `sudo btrfs rest -m -x -S -o percorso/al/device/btrfs percorso/alla/directory_target` : ripristina file da un filesystem btrfs (insieme a metadati, attributi estesi e Symlink), sovrascrivendo i file nel target

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-restore.html
