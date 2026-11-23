# docker container cp

Copie des fichiers ou des répertoires entre les systèmes de fichiers hôte et conteneur.

## Usage
- `docker cp chemin/vers/fichier_ou_répertoire_de_l_hote nom_du_conteneur:chemin/vers/fichier_ou_répertoire_du_conteneur` : copie un fichier ou un répertoire de l'hôte vers un conteneur
- `docker cp nom_du_conteneur:chemin/vers/fichier_ou_répertoire_du_conteneur chemin/vers/fichier_ou_répertoire_de_l_hote` : copie un fichier ou un répertoire d'un conteneur vers l'hôte
- `docker cp -L chemin/vers/lien_symbolique_de_l_hote nom_du_conteneur:chemin/vers/fichier_ou_répertoire_du_conteneur` : copie un fichier ou un répertoire de l'hôte vers un conteneur, en suivant les liens symboliques (copie les fichiers liés directement, pas les liens symboliques eux-mêmes)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/cp/
