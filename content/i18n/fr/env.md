# env

Affiche l'environnement ou exécute un programme dans un environnement modifié.

## Usage
- `env` : affiche l'environnement
- `env programme` : exécute le programme donné. Souvent utilisé dans les scripts après le shebang (`#!`) pour consulter le chemin vers le programme
- `env -i programme` : exécute le programme donné, avec un environnement vide
- `env -u variable programme` : supprime une variable d'environnement et execute le programme donné
- `env variable=valeur programme` : définit ou modifie une variable d'environnement et execute le programme donné
- `env variable1=valeur variable2=valeur variable3=valeur programme` : définit ou modifie plusieurs variables d'environnement et execute le programme donné

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/env-invocation.html
