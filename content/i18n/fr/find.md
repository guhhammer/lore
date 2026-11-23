# find

Trouve récursivement des fichiers ou des dossiers dans l'arborescence spécifiée.

## Usage
- `find chemin/vers/répertoire -name '*.ext'` : trouve des fichiers par extension
- `find chemin/vers/répertoire -path '*/chemin/*/*.ext' -or -name '*motif*'` : trouve des fichiers correspondant à plusieurs chemins ou motifs
- `find chemin/vers/répertoire -type d -iname '*lib*'` : trouve des dossiers correspondant à un nom donné sans vérifier la casse
- `find chemin/vers/répertoire -name '*.py' -not -path '*/site-packages/*'` : trouve des fichiers correspondant à un motif donné en excluant certains chemins de la recherche
- `find chemin/vers/répertoire -maxdepth 1 -size +500k -size -10M` : trouve des fichiers dans une fourchette de tailles en limitant la profondeur récursive à "1"
- `find chemin/vers/répertoire -name '*.ext' -exec wc -l {} \;` : exécute une commande pour chaque fichier (utiliser `{}` dans la commande pour utiliser le nom des fichiers)
- `find chemin/vers/répertoire -daystart -mtime -1 -exec tar -cvf archive.tar {} \+` : trouve les fichiers modifiés aujourd'hui et transmet les résultats à une commande comme arguments
- `find chemin/vers/répertoire -type f|d -empty -delete -print` : trouve les fichiers ou les répertoires vides et les supprime en affichant les résultats

## Links
- More information: https://manned.org/find
