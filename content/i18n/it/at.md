# at

Esegue comandi una sola volta in un momento futuro. I risultati vengono inviati alla mail dell'utente.

## Usage
- `at now + 5 minutes` : crea comandi interattivamente ed eseguili tra 5 minuti (premi `<Ctrl d>` quando finito)
- `at hh:mm` : crea comandi interattivamente ed eseguili a un orario specifico
- `echo "comando" | at 1000` : esegue un comando da `stdin` alle 10:00 di oggi
- `at -f percorso/del/file 9:30 PM Tue` : esegue comandi da un file specifico il prossimo martedì
- `at -l` : elenca tutti i job in coda per l'utente corrente (uguale a `atq`)
- `at -c numero_job` : visualizza un job specifico

## Links
- More information: https://manned.org/at
