# btrfs property

Ottiene, imposta o elenca proprietà per oggetti BTRFS (file, directory, sottovolumi, filesystem o dispositivi).

## Usage
- `sudo btrfs p l percorso/all/oggetto_btrfs` : elenca le proprietà disponibili (e descrizioni) per l'oggetto btrfs dato
- `sudo btrfs p g percorso/all/oggetto_btrfs` : ottiene tutte le proprietà per l'oggetto btrfs dato
- `sudo btrfs p g percorso/al/filesystem_btrfs label` : ottiene la proprietà `label` per il filesystem o dispositivo btrfs dato
- `sudo btrfs p g -t subvol|filesystem|inode|device percorso/al/filesystem_btrfs` : ottiene tutte le proprietà specifiche del tipo di oggetto per il filesystem o dispositivo btrfs dato
- `sudo btrfs p s percorso/all/inode_btrfs compression zstd|zlib|lzo|none` : imposta la proprietà `compression` per un inode btrfs dato (file o directory)

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-property.html
