# btrfs rescue

Tenta di recuperare un filesystem btrfs danneggiato.

## Usage
- `sudo btrfs resc ch percorso/alla/partizione` : ricostruisce l'albero dei metadati del filesystem (molto lento)
- `sudo btrfs resc fix-de percorso/alla/partizione` : corregge problemi di allineamento della dimensione del dispositivo (es. impossibilità di montare il filesystem con mismatch dei byte totali del superblock)
- `sudo btrfs resc s percorso/alla/partizione` : recupera un superblock corrotto dalle copie corrette (recupera la radice dell'albero del filesystem)
- `sudo btrfs resc z percorso/alla/partizione` : recupera da transazioni interrotte (corregge problemi di replay del log)
- `sudo btrfs resc c` : crea un dispositivo di controllo `/dev/btrfs-control` quando `mknod` non è installato

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-rescue.html
