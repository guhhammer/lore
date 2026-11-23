# truss

Outil de dépannage pour tracer les appels système. Équivalent SunOS de strace.

## Usage
- `truss -f programme` : commencez à tracer un programme en l'exécutant, en suivant tous les processus enfants
- `truss -p pid` : commencez à tracer un processus spécifique par son PID
- `truss -a -e programme` : commencez à tracer un programme en l'exécutant, en affichant les arguments et les variables d'environnement
- `truss -c -p pid` : comptez le temps, les appels et les erreurs pour chaque appel système et rapportez un résumé à la sortie du programme
- `truss -p pid -t nom_d'appel_système` : tracez une sortie de filtrage de processus par appel système

## Links
- More information: https://www.unix.com/man-page/sunos/1/truss
