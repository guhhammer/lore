# chroot

Esegui un comando o una shell interattiva con una speciale directory root.

## Usage
- `sudo chroot percorso/della/nuova/root` : esegui `$SHELL` nella nuova directory root
- `sudo chroot percorso/della/nuova/root comando` : esegui un comando nella nuova directory root
- `sudo chroot --userspec nome_utente_o_id:nome_gruppo_o_id percorso/della/nuova/root` : usa un utente e un gruppo specifici
