# diff

Compare deux fichiers ou répertoires.

## Usage
- `diff ancien_fichier nouveau_fichier` : compare deux fichiers (liste les changements pour transformer `ancien_fichier` en `nouveau_fichier`)
- `diff -w ancien_fichier nouveau_fichier` : compare deux fichiers, en ignorant l'espacement
- `diff -y ancien_fichier nouveau_fichier` : compare deux fichiers, en affichant différences côte à côte
- `diff -u ancien_fichier nouveau_fichier` : compare deux fichiers, en affichant les différences dans un format unifié (comme `git diff`)
- `diff -r ancien_répertoire nouveau_répertoire` : compare récursivement deux répertoires directories (affiche les noms de fichiers et de répertoires divergents ainsi que les modifications de fichiers)
- `diff -r -q ancien_répertoire nouveau_répertoire` : compare deux répertoires, en affichant uniquement les fichiers qui diffèrent
- `diff -a -u -N ancien_fichier nouveau_fichier >diff.patch` : crée un fichier patch des différences entre deux fichiers texte pour Git, en traitant les fichiers inexistants comme fichiers vides

## Links
- More information: https://manned.org/diff
