# docker secret

Gére les secrets de Docker swarm.

## Usage
- `commande | docker secret create nom_du_secret -` : crée un nouveau secret depuis l'entrée standard
- `docker secret create nom_du_secret chemin/vers/fichier` : crée un nouveau secret depuis un fichier
- `docker secret ls` : liste tous les secrets
- `docker secret inspect --pretty nom_du_secret1 nom_du_secret2 ...` : affiche des informations détaillées sur un ou plusieurs secrets dans un format humain
- `docker secret rm nom_du_secret1 nom_du_secret2 ...` : supprime un ou plusieurs secrets

## Links
- More information: https://docs.docker.com/reference/cli/docker/secret/
