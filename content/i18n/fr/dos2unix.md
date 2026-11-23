# dos2unix

Remplace les fins de lignes de style DOS par des fins de lignes de style Unix. Remplace CRLF par LF.

## Usage
- `dos2unix chemin/vers/fichier` : remplace les fins de lignes d'un fichier
- `dos2unix -n chemin/vers/fichier chemin/vers/nouveau_fichier` : crée une copie avec des fins de lignes de type Unix
- `dos2unix -i chemin/vers/fichier` : affiche les informations d'un fichier
- `dos2unix --keep-bom|add-bom|remove-bom chemin/vers/fichier` : conserve/Ecrit/Supprime la marque d'ordre des octets (BOM)

## Links
- More information: https://manned.org/dos2unix
