# log

Visualizza, esporta e configura i sistemi di log.

## Usage
- `log stream` : mostra i log di sistema in diretta
- `log stream --process id_processo` : mostra i log mandati a `syslog` da processi con un PID specifico
- `log show --predicate "process == 'process_name'"` : mostra i log mandati a `syslog` da processi con un nome specifico
- `sudo log collect --last 1h --output percorso/del/file.logarchive` : esporta sul disco tutti i log dell'ultima ora

## Links
- More information: https://keith.github.io/xcode-man-pages/log.1.html
