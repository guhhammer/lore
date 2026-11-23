# aa-audit

Imposta i profili di sicurezza AppArmor in modalità audit.

## Usage
- `sudo aa-audit nome_profilo` : imposta un profilo in modalità audit
- `sudo aa-audit profilo1 profilo2 ...` : imposta più profili in modalità audit
- `sudo aa-audit -d /path/to/profiles nome_profilo` : imposta un profilo in modalità audit da una directory specifica
- `sudo aa-audit --force nome_profilo` : forza la modalità audit anche se già applicata
- `sudo aa-audit --no-reload nome_profilo` : imposta un profilo in modalità audit senza ricaricarlo
- `sudo aa-audit -r nome_profilo` : rimuove la modalità audit per un profilo
- `aa-audit -h` : mostra l'aiuto

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-audit.8
