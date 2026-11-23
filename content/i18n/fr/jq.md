# jq

Un processeur JSON en ligne de commande qui utilise un langage dédié (DSL).

## Usage
- `cat chemin/vers/fichier.json | jq '.'` : exécute une expression spécifique (affiche une sortie JSON coloré et formaté)
- `cat chemin/vers/fichier.json | jq --from-file chemin/vers/script.jq` : exécute un script spécifique
- `cat chemin/vers/fichier.json | jq --arg "nom1" "valeur1" --arg "nom2" "valeur2" ... '. + $ARGS.named'` : transmet des arguments spécifiques
- `cat chemin/vers/fichier.json | jq '.clé1, .clé2, ...'` : imprime des clés spécifiques
- `cat chemin/vers/fichier.json | jq '.[index1], .[index2], ...'` : imprime des éléments spécifiques du tableau
- `cat chemin/vers/fichier.json | jq '.[]'` : imprime tous les éléments du tableau/les clés de l'objet
- `cat chemin/vers/fichier.json | jq '. +|- {"clé1": "valeur1", "clé2": "valeur2", ...}'` : ajoute/supprime des clés spécifiques

## Links
- More information: https://jqlang.org/manual/
