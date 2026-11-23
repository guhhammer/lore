# aa-status

Elenca i moduli AppArmor attualmente caricati.

## Usage
- `sudo aa-status` : controlla lo stato
- `sudo aa-status --json` : mostra lo stato in formato JSON
- `sudo aa-status --pretty-json` : mostra lo stato in formato JSON con formattazione
- `sudo aa-status --profiled` : mostra il numero di policy caricate
- `sudo aa-status --enforced` : mostra il numero di policy in enforce caricate
- `sudo aa-status --complaining` : mostra il numero di policy non in enforce caricate
- `sudo aa-status --kill` : mostra il numero di policy in enforce che terminano i processi

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-status.8
