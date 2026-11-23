# aa-genprof

Genera profili di sicurezza AppArmor monitorando il comportamento dei programmi.

## Usage
- `sudo aa-genprof percorso/del/programma` : avvia la generazione di un profilo per un programma
- `sudo aa-genprof -d /path/to/profiles percorso/del/programma` : specifica una directory personalizzata per i profili
- `sudo aa-genprof -f /path/to/logfile percorso/del/programma` : specifica un file di log personalizzato per la profilazione
- `aa-genprof -h` : mostra l'aiuto

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-genprof.8
