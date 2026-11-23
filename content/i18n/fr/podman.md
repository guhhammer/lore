# podman

Outil de gestion simple pour les pods, conteneurs et images. Podman fournit une interface en ligne de commande comparable à Docker. En résumé : `alias docker=podman`.

## Usage
- `podman ps -a` : liste tous les conteneurs (en cours d'exécution et arrêtés)
- `podman run --name nom_conteneur image` : crée un conteneur à partir d'une image, avec un nom personnalisé
- `podman start|stop nom_conteneur` : démarre ou arrête un conteneur existant
- `podman pull image` : télécharge une image depuis un registre (par défaut Docker Hub)
- `podman images` : affiche la liste des images déjà téléchargées
- `podman exec -it nom_conteneur sh` : ouvre un shell dans un conteneur déjà en cours d'exécution
- `podman rm nom_conteneur` : supprime un conteneur arrêté
- `podman logs -f nom_conteneur identifiant_conteneur ...` : affiche les journaux d'un ou plusieurs conteneurs et suit la sortie en continu

## Links
- More information: https://docs.podman.io/en/latest/Commands.html
