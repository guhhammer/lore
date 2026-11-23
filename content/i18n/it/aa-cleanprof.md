# aa-cleanprof

Pulisce i profili di sicurezza AppArmor rimuovendo le regole inutilizzate.

## Usage
- `sudo aa-cleanprof nome_profilo` : pulisce un profilo per rimuovere le regole inutilizzate
- `sudo aa-cleanprof profilo1 profilo2 ...` : pulisce più profili contemporaneamente
- `sudo aa-cleanprof -d /path/to/profiles nome_profilo` : specifica la directory contenente i profili
- `sudo aa-cleanprof -s nome_profilo` : esegue silenziosamente senza richieste
- `sudo aa-cleanprof --no-reload nome_profilo` : impedisce il ricaricamento del profilo dopo la pulizia
- `aa-cleanprof -h` : mostra l'aiuto

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-cleanprof.8
