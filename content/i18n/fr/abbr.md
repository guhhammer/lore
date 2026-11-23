# abbr

Gère les abréviations pour le shell fish. Les mots définis par l'utilisateur sont remplacés par des phrases plus longues après leur saisie.

## Usage
- `abbr -a nom_abrégé commande arguments_de_la_commande` : ajoute une nouvelle abréviation
- `abbr --rename ancien_nom nouveau_nom` : renomme une abréviation existante
- `abbr -e nom_abrégé` : supprime une abréviation existante
- `ssh nom_de_l_hôte abbr -s | source` : importe les abréviations définies sur un autre hôte via SSH

## Links
- More information: https://fishshell.com/docs/current/cmds/abbr.html
