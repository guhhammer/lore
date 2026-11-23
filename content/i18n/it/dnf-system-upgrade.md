# dnf system-upgrade

Aggiorna Fedora a una nuova versione maggiore.

## Usage
- `sudo dnf upgrade --refresh` : prepara il sistema aggiornando tutti i pacchetti installati
- `sudo dnf system-upgrade download --releasever release_version` : scarica i pacchetti richiesti per la nuova versione di release
- `sudo dnf system-upgrade reboot` : riavvia e avvia il processo di aggiornamento
- `sudo dnf system-upgrade status` : mostra lo stato della transazione di aggiornamento corrente
- `sudo dnf system-upgrade log` : mostra i log dei tentativi di aggiornamento precedenti
- `sudo dnf system-upgrade clean` : rimuove i dati e i pacchetti di aggiornamento in cache

## Links
- More information: https://dnf5.readthedocs.io/en/latest/commands/system-upgrade.8.html
