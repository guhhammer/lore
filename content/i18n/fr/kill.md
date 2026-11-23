# kill

Envoie un signal à un processus, généralement pour l'interrompre. Tous les signaux sauf `SIGKILL` et `SIGSTOP` peuvent être interceptés par le processus pour pouvoir se terminer proprement.

## Usage
- `kill identifiant_processus` : termine un processus avec le signal `SIGTERM` (terminaison) par défaut
- `kill -l` : liste les noms des signaux disponibles (à utiliser sans leurs préfixes `SIG`)
- `kill -1 identifiant_processus` : termine un processus avec le signal `SIGHUP` ("raccrocher"). Beaucoup de daemons se rafraîchissent au lieu de terminer
- `kill -2 identifiant_processus` : termine un processus avec le signal `SIGINT` ("interruption"). Généralement initié par l'utilisateur appuyant sur `<Ctrl c>`
- `kill -9 identifiant_processus` : demande au système d'exploitation de mettre fin à un processus immédiatement (sans possibilité d'intercepter le signal)
- `kill -STOP identifiant_processus` : demande au système d'exploitation de suspendre un processus jusqu'à recevoir le signal `SIGCONT` ("continue")
- `kill -USR1 -identifiant_groupe` : envoie le signal `SIGUSR1` à tous les processus dans le groupe avec l'identifiant indiqué

## Links
- More information: https://manned.org/kill.1posix
