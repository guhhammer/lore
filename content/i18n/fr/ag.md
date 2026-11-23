# ag

The Silver Searcher. Comme `ack`, mais inspire à être plus rapide.

## Usage
- `ag string` : trouve les fichiers qui contiennent `string`, et affiche les lignes correspondantes dans le contexte courant
- `ag string chemin/vers/répertoire` : trouve les fichiers qui contiennent `string` dans un répertoire spécifique
- `ag -l string` : trouve les fichiers qui contiennent `string`, mais affiche uniquement les noms des fichiers
- `ag -i -o STRING` : trouve les fichiers qui contiennent `STRING` en étant insensible à la casse et affiche que le résultat plutôt que la ligne entière
- `ag string -G nom_fichier` : trouve `string` dans les fichiers nommés `nom_fichier`
- `ag '^ca(t|r)$'` : trouve des fichiers dont le contenu correspond à une `regex`
- `ag -g string` : trouve les fichiers avec un nom correspondant à `string`

## Links
- More information: https://manned.org/ag
