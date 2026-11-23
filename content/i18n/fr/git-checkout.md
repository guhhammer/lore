# git checkout

Extrait une branche ou des chemins vers l'arborescence de travail.

## Usage
- `git checkout -b nom_branche` : crée une branche et bascule dessus
- `git checkout -b nom_branche référence` : crée une branche depuis une référence spécifique et bascule dessus (par exemple, branche locale/distante, étiquette, validation)
- `git checkout nom_branche` : bascule sur une branche locale existante
- `git checkout -` : bascule sur la branche précédente
- `git checkout -t nom_distant/nom_branche` : bascule sur une branche distante existante
- `git checkout .` : annule tout les changements dans le répertoire actuel (voir `git reset` pour plus de commandes d'annulation)
- `git checkout chemin/vers/fichier` : annule tout les changements dans le fichier spécifié
- `git checkout nom_branche -- chemin/vers/fichier` : remplace un fichier par sa version d'une autre branche

## Links
- More information: https://git-scm.com/docs/git-checkout
