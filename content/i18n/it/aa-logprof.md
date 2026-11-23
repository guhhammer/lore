# aa-logprof

Aggiorna interattivamente i profili di sicurezza AppArmor basandosi sulle violazioni registrate.

## Usage
- `sudo aa-logprof` : esamina e aggiorna interattivamente i profili basandosi sui log di sistema
- `sudo aa-logprof -d /path/to/profiles` : usa una directory specifica per i profili AppArmor
- `sudo aa-logprof -f /path/to/logfile` : usa un file di log specifico invece di quello predefinito
- `sudo aa-logprof -m "testo_marcatore_log"` : ignora tutte le voci di log precedenti al marcatore specificato
- `aa-logprof -h` : mostra l'aiuto

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-logprof.8
