# pveum

Gestisce gli utenti di Proxmox.

## Usage
- `pveum u l` : elenca gli utenti
- `pveum u a nome_utente@pve` : aggiungi un utente
- `pveum u a nome_utente@pve --email indirizzo_email --comment descrizione --password password` : aggiungi un utente con email, descrizione e password
- `pveum pa nome_utente@pve` : cambia password utente
- `pveum u d nome_utente@pve` : elimina un utente
- `pveum a m /vms/1000 --user nome_utente@pve --role PVEVMUser` : dai a un utente accesso a una VM specifica con permessi specifici

## Links
- More information: https://pve.proxmox.com/pve-docs/pveum.1.html
