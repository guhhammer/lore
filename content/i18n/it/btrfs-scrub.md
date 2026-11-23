# btrfs scrub

Scrub dei filesystem btrfs per verificare l'integrità dei dati. Si consiglia di eseguire uno scrub una volta al mese.

## Usage
- `sudo btrfs sc start percorso/al/mount_btrfs` : avvia uno scrub
- `sudo btrfs sc status percorso/al/mount_btrfs` : mostra lo stato di uno scrub in corso o dell'ultimo completato
- `sudo btrfs sc c percorso/al/mount_btrfs` : annulla uno scrub in corso
- `sudo btrfs sc r percorso/al/mount_btrfs` : riprende uno scrub precedentemente annullato
- `sudo btrfs sc start -B percorso/al/mount_btrfs` : avvia uno scrub, ma non mette il programma in Background
- `sudo btrfs sc start -q percorso/al/mount_btrfs` : avvia uno scrub in modalità silenziosa (non stampa errori o statistiche)

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-scrub.html
