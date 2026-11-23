# autossh

Esegue, monitora e riavvia connessioni SSH. Si riconnette automaticamente per tenere attivi tunnel di port forwarding. Accetta tutte le flag di SSH.

## Usage
- `autossh -M porta_monitorata "comando_ssh"` : apri una sessione SSH, riavviandola quando una porta monitorata smette di rispondere
- `autossh -M porta_monitorata -L porta_locale:localhost:porta_remota utente@host` : apri una sessione SSH che forwarda una porta locale verso una remota, riavviandola se necessario
- `autossh -f -M porta_monitorata -N "comando_ssh"` : forka prima di eseguire il comando SSH (si avvia in background) e non aprire una shell remota
- `autossh -f -M 0 -N -o "ServerAliveInterval 10" -o "ServerAliveCountMax 3" "comando_ssh"` : esegui autossh in background, senza una porta da monitorare, utilizzando i keep-alive di SSH ogni 10 secondi per rilevare una disconnessione
- `autossh -f -M 0 -N -o "ServerAliveInterval 10" -o "ServerAliveCountMax 3" -o ExitOnForwardFailure=yes -L porta_locale:localhost:porta_remota utente@host` : esegui autossh in background, senza una porta da monitorare, senza una shell remota, uscendo se il port forwarding fallisce
- `AUTOSSH_DEBUG=1 AUTOSSH_LOGFILE=file_log autossh -f -M porta_monitorata -v -E file_log_ssh comando_ssh` : esegui autossh in background con output di debug su un file e output verboso di ssh su un altro file

## Links
- More information: https://manned.org/autossh
