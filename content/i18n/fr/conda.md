# conda

Gestion des paquets, des dépendances et des environnements pour tout langage de programmation. Certaines sous-commandes comme `create` ont leur propre documentation.

## Usage
- `conda create -n nom_environnement python=3.9 matplotlib` : crée un nouvel environnement en y installant les paquets nommés
- `conda info -e` : liste tous les environnements
- `conda activate nom_environnement` : active un environnement
- `conda deactivate` : désactive un environnement
- `conda remove -n nom_environnement --all` : supprime un environnement (supprime tous les paquets)
- `conda install python=3.4 numpy` : installe des paquets dans l'environnement courant
- `conda list` : liste les paquets actuellement installés dans l'environnement courant
- `conda clean -a` : supprime les paquets et les caches inutilisés

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/index.html
