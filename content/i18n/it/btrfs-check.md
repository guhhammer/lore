# btrfs check

Controlla o ripara un filesystem btrfs.

## Usage
- `sudo btrfs c percorso/alla/partizione` : controlla un filesystem btrfs
- `sudo btrfs c --repair percorso/alla/partizione` : controlla e ripara un filesystem btrfs (pericoloso)
- `sudo btrfs c -p percorso/alla/partizione` : mostra il progresso del controllo
- `sudo btrfs c --check-data-csum percorso/alla/partizione` : verifica il checksum di ogni blocco dati (se il filesystem è integro)
- `sudo btrfs c -s n percorso/alla/partizione` : usa il `n`-esimo superblock (`n` può essere 0, 1 o 2)
- `sudo btrfs c --repair --init-csum-tree percorso/alla/partizione` : ricostruisce l'albero dei checksum
- `sudo btrfs c --repair --init-extent-tree percorso/alla/partizione` : ricostruisce l'albero degli extent

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-check.html
