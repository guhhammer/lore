# svcs

Répertorie les informations sur les services en cours d'exécution.

## Usage
- `svcs` : liste tous les services en cours d'exécution
- `svcs -vx` : liste les services qui ne fonctionnent pas
- `svcs apache` : répertorie les informations sur un service
- `svcs -L apache` : affiche l'emplacement du fichier journal de service
- `tail $(svcs -L apache)` : affiche la fin d'un fichier journal de service

## Links
- More information: https://www.unix.com/man-page/sunos/1/svcs
