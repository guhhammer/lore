# ack

Un outil de recherche comme grep, optimisé pour les développeurs.

## Usage
- `ack "motif_de_recherche"` : recherche des fichiers contenant une chaine de caractère ou une `regex` dans le répertoire courant récursivement
- `ack -i "motif_de_recherche"` : recherche pour un motif insensible à la casse
- `ack -o "motif_de_recherche"` : recherche les lignes qui correspondent à un motif, affiche uniquement les textes correspondants et pas le reste de la ligne
- `ack -t ruby "motif_de_recherche"` : limite la recherche aux fichiers d'un certain type
- `ack -t noruby "motif_de_recherche"` : exlcus un certain type de fichier de la recherche
- `ack -c -h "motif_de_recherche"` : compte le nombre total de correspondances
- `ack -c -l "motif_de_recherche"` : affiche les noms et le nombre total de correspondances pour chaque fichiers
- `ack --help-types` : affiche toutes les valeurs qui peuvent être utilisées avec `--type`

## Links
- More information: https://beyondgrep.com/documentation/
