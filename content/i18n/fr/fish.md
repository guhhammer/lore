# fish

Friendly Interactive SHell, un interpréteur de ligne de commande, conçu pour être facile à utiliser.

## Usage
- `fish` : démarre une session shell interactive
- `fish -c "commande"` : exécute une commande, puis termine la session
- `fish chemin/vers/le/script.fish` : exécute un script
- `fish -N chemin/vers/le/script.fish` : vérifie les erreurs de syntaxe dans un script
- `fish -P` : démarre une session shell interactive en mode privé, dans laquelle le shell n'a pas accès à l'historique et n'y écrit rien
- `fish -v` : affiche les informations de version
- `set -Ux nom_de_la_variable valeur_de_la_variable` : ajoute et exporte une variable d'environnement, qui persiste entre les redémarrages du shell (à exécuter depuis le shell uniquement)

## Links
- More information: https://fishshell.com/docs/current/cmds/fish.html
