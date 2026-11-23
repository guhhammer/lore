# wc

Compte les lignes, les mots ou les octets.

## Usage
- `wc -l chemin/vers/fichier` : compte les lignes d'un fichier
- `wc -w chemin/vers/fichier` : compte les mots d'un fichier
- `wc -c chemin/vers/fichier` : compte les octets d'un fichier
- `wc -m chemin/vers/fichier` : compte les caractères d'un fichier (en prenant en compte l'ensemble des caractères multi-octets)
- `find . | wc` : compte les lignes, les mots et les caractères depuis l'entrée standard `stdin`
- `wc -L chemin/vers/fichier` : compte la longueur en nombre de caractères de la plus grande ligne d'un fichier

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/wc-invocation.html
