# crontab

Programma cron job per essere eseguiti a determinati intervalli di tempo per l'utente corrente. Formato definizione di un job: "(minuto) (ora) (giorno_del_mese) (mese) (giorno_della_settimana) comando_da_eseguire".

## Usage
- `crontab -e` : modifica il file crontab per l'utente corrente
- `crontab -l` : elenca i cron job esistenti per l'utente corrente
- `crontab -r` : rimuovi tutti i cron job per l'utente corrente
- `0 10 * * * comando_da_eseguire` : esempio di un job eseguito alle 10:00 ogni giorno (* vuol dire qualsiasi valore)
- `* * 3 Apr * comando_da_eseguire` : esempio di un job eseguito ogni minuto il 3 Aprile
- `30 2 * * Fri /percorso/assoluto/dello/script.sh` : esempio di un job che esegue un determinato script alle 02:30 ogni venerdì

## Links
- More information: https://manned.org/crontab
