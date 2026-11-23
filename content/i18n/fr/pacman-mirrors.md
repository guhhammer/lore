# pacman-mirrors

Génère une liste de miroirs pour pacman sur Manjaro Linux. Tout appel à pacman-mirrors demande de synchroniser les bases de données et de mettre à jour le système en exécutant `sudo pacman -Syyu` en suivant.

## Usage
- `sudo pacman-mirrors --fasttrack` : génère une liste de miroirs avec les réglages par défaut
- `pacman-mirrors --status` : obtiens le statut des miroirs actuels
- `pacman-mirrors --get-branch` : affiche la branche courante
- `sudo pacman-mirrors --api --set-branch stable|unstable|testing` : utilise une branche différente
- `sudo pacman-mirrors --geoip` : génère une liste de miroirs se trouvant dans le pays associé à l'adresse IP

## Links
- More information: https://wiki.manjaro.org/index.php?title=Pacman-mirrors
