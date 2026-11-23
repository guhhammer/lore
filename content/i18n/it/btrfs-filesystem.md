# btrfs filesystem

Gestisce i filesystem btrfs.

## Usage
- `btrfs f u percorso/al/mount_btrfs` : mostra l'utilizzo del filesystem (opzionalmente esegui come root per informazioni dettagliate)
- `sudo btrfs f sh percorso/al/mount_btrfs` : mostra l'utilizzo per dispositivi individuali
- `sudo btrfs f de -v percorso/del/file` : deframmenta un singolo file su un filesystem btrfs (evita durante l'esecuzione di un agente di deduplicazione)
- `sudo btrfs f de -v -r percorso/della/directory` : deframmenta una directory ricorsivamente (non attraversa i confini dei sottovolumi)
- `sudo btrfs f sy percorso/al/mount_btrfs` : forza la sincronizzazione dei blocchi dati non scritti su disco(i)
- `sudo btrfs f du -s percorso/della/directory` : riassume l'utilizzo del disco per i file in una directory ricorsivamente
- `sudo btrfs f m --size 8g --uuid clear|random|time|uuid_value percorso/allo/swapfile` : crea un file di swap

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-filesystem.html
