# as

Assembleur GNU portable. Principalement destiné pour assembler la sortie de `gcc` pour être utilisé par `ld`.

## Usage
- `as chemin/vers/fichier.s` : assemble un fichier, en écrivant la sortie dans le fichier `a.out`
- `as chemin/vers/fichier.s -o chemin/vers/sortie.o` : assemble la sortie vers un fichier donné
- `as -f chemin/vers/fichier.s` : génère la sortie plus vite en évitant le preprocess des espaces et des commentaires (doit seulement être utilisé sur des compilateurs sûrs)
- `as -I chemin/vers/répertoire chemin/vers/fichier.s` : inclut un chemin donné à la liste des répertoires dans lesquels chercher les fichiers spécifiés dans les directives `.include`

## Links
- More information: https://manned.org/as
