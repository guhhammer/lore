# dnf download

Scarica pacchetti RPM dai repository DNF. Non predefinito in `dnf` ma supportato tramite `dnf-plugins-core`.

## Usage
- `dnf download pacchetto` : scarica l'ultima versione di un pacchetto nella directory corrente
- `dnf download pacchetto --destdir percorso/della/directory` : scarica un pacchetto in una directory specifica (la directory deve esistere)
- `dnf download --url pacchetto` : stampa l'URL da cui scaricare il pacchetto RPM

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/download.html
