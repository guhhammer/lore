# deluser

Supprime un utilisateur du système.

## Usage
- `sudo deluser nom_dutilisateur` : supprime un utilisateur
- `sudo deluser --remove-home nom_dutilisateur` : supprime un utilisateur et son répertoire "home"
- `sudo deluser --backup-to chemin/vers/le/répertoire/de/sauvegarde --remove-home nom_dutilisateur` : supprime un utilisateur et son répertoire "home", mais sauvegarde ses fichiers dans une archive `.tar.gz` dans le répertoire spécifié
- `sudo deluser --remove-all-files nom_dutilisateur` : supprime un utilisateur et tous les fichiers qu'il possède

## Links
- More information: https://manned.org/deluser
