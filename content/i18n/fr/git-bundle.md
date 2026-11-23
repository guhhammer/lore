# git bundle

Empaquete des objets et des références dans une archive.

## Usage
- `git bundle create chemin/vers/fichier.bundle nom_branche` : empaquete tout les objets et les références d'une branche spécifiée
- `git bundle create chemin/vers/fichier.bundle --all` : crée un empaquetage de tout les fichiers de toutes les branches
- `git bundle create chemin/vers/fichier.bundle -5 HEAD` : crée un empaquetage des 5 derniers commits de la branche courante
- `git bundle create chemin/vers/fichier.bundle --since 7.days HEAD` : crée un empaquetage des 7 derniers jours
- `git bundle verify chemin/vers/fichier.bundle` : vérifie qu'un empaquetage est valide et peut être appliqué à la branche courante
- `git bundle unbundle chemin/vers/fichier.bundle` : affiche sur la sortie standard la liste des références contenues dans un empaquetage
- `git pull chemin/vers/fichier.bundle nom_branche` : extrait une branche spécifique d'un fichier de bundle dans le référentiel actuel
- `git clone chemin/vers/fichier.bundle` : crée un nouveau dépôt depuis un empaquetage

## Links
- More information: https://git-scm.com/docs/git-bundle
