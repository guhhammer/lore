# aa-unconfined

Elenca i processi con porte TCP/UDP aperte che non hanno profili AppArmor caricati.

## Usage
- `sudo aa-unconfined` : elenca i processi non confinati usando il comando `ss` (predefinito)
- `sudo aa-unconfined --with-netstat` : usa `netstat` invece di `ss` per rilevare i socket di rete aperti
- `sudo aa-unconfined --paranoid` : mostra tutti i processi da `/proc` con porte TCP/UDP e nessun profilo AppArmor (più dettagliato)
- `aa-unconfined -h` : mostra l'aiuto

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-unconfined.8
