# pkgctl repo

Gestisce repository Git di pacchetti e la loro configurazione per Arch Linux.

## Usage
- `pkgctl repo clone nome_pkg` : clona un repository di pacchetti (richiede l'impostazione di una chiave SSH nell'account GitLab di Arch Linux)
- `pkgctl repo clone --protocol https nome_pkg` : clona un repository di pacchetti tramite HTTPS
- `pkgctl repo create pkgbase` : crea un nuovo repository di pacchetti GitLab e lo clona dopo la creazione (richiede autenticazione API GitLab valida)
- `pkgctl repo switch versione pkgbase` : cambia un repository di pacchetti a una versione specificata
- `pkgctl repo web pkgbase` : apre il sito web di un repository di pacchetti

## Links
- More information: https://manned.org/pkgctl-repo
