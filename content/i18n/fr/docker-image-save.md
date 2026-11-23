# docker image save

Exporte une ou plusieurs images Docker dans une archive.

## Usage
- `docker save image:étiquette >chemin/vers/fichier.tar` : sauvegarde une image en redirigeant la sortie standard vers une archive `.tar`
- `docker save -o chemin/vers/fichier.tar image:étiquette` : sauvegarde une image dans une archive `.tar`
- `docker save -o chemin/vers/fichier.tar nom_image` : sauvegarde toutes les étiquettes de l'image
- `docker save -o chemin/vers/fichier.tar nom_image:étiquette1 nom_image:étiquette2 ...` : sélectionne des étiquettes particulières d'une image à sauvegarder

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/save/
