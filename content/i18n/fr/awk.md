# awk

Langage de programmation polyvalent pour travailler sur des fichiers. Remarque : Différentes implémentations d'AWK font souvent de ceci un lien symbolique vers leur binaire.

## Usage
- `awk '{print $5}' chemin/vers/fichier` : affiche la cinquième colonne (ou le champ) dans un fichier qui utilise des espaces comme séparateur
- `awk '/string1/ {print $2}' chemin/vers/fichier` : affiche la deuxième colonne des lignes contenant "string1" dans un fichier qui utilise des espaces comme séparateur
- `awk -F ',' '{print $NF}' chemin/vers/fichier` : affiche la dernière colonne de chaque ligne d'un fichier en utilisant une virgule (au lieu des espaces) comme séparateur
- `awk '{s+=$1} END {print s}' chemin/vers/fichier` : additionne les valeurs de la première colonne des lignes d'un fichier et affiche le total
- `awk 'NR%3==1' chemin/vers/fichier` : affiche une ligne sur trois en partant de la première ligne
- `awk '{if ($1 == "string1") print "Correspondance exacte string1"; else if ($1 ~ "string2") print "Correspondance partielle string2"; else print "string3"}' chemin/vers/fichier` : affiche différentes valeurs selon des conditions
- `awk '($10 >= valeur_min && $10 <= valeur_max)' chemin/vers/fichier` : affiche les lignes dont la valeur de la 10ème colonne est comprise entre un min et un max
- `awk 'BEGIN {FS=":";printf "%-20s %6s %25s\n", "Name", "UID", "Shell"} $4 >= 1000 {printf "%-20s %6d %25s\n", $1, $4, $7}' /etc/passwd` : affiche une table des utilisateurs avec un UID >= 1000, avec un en-tête et une sortie formatée, en utilisant les deux-points comme séparateur (`%-20s` : 20 caractères alignés à gauche, `%6s` : 6 caractères alignés à droite)

## Links
- More information: https://github.com/onetrueawk/awk
