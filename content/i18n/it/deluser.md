# deluser

Elimina un utente dal sistema.

## Usage
- `sudo deluser nome_utente` : rimuove un utente
- `sudo deluser --remove-home nome_utente` : rimuove un utente e la sua directory home
- `sudo deluser --backup-to percorso/alla/directory_di_backup --remove-home nome_utente` : rimuove un utente e la sua home, ma salva i file in un `.tar.gz` nella directory specificata
- `sudo deluser --remove-all-files nome_utente` : rimuove un utente e tutti i file di sua proprietà

## Links
- More information: https://manned.org/deluser
