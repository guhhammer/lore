# docker login

Se connecte à un registre Docker.

## Usage
- `docker login` : se connecte de manière interactive à un registre
- `docker login -u nom_utilisateur` : se connecte à un registre avec un nom d'utilisateur spécifique (l'utilisateur sera invité à saisir un mot de passe)
- `docker login -u nom_utilisateur -p mot_de_passe serveur` : se connecte à un registre avec un nom d'utilisateur et un mot de passe spécifiques
- `echo "mot_de_passe" | docker login -u nom_utilisateur --password-stdin` : se connecte à un registre avec un mot de passe depuis l'entrée standard

## Links
- More information: https://docs.docker.com/reference/cli/docker/login/
