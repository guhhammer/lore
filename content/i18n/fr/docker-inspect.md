# docker inspect

Retour d'informations de bas niveau sur les objets Docker.

## Usage
- `docker inspect conteneur|image|id` : affiche les informations de configuration d'un conteneur, image ou volume en utilisant un nom ou un ID
- `docker inspect -f '\{\{range.NetworkSettings.Networks\}\}\{\{.IPAddress\}\}\{\{end\}\}' conteneur` : affiche l'adresse IP d'un conteneur
- `docker inspect -f '\{\{.LogPath\}\}' conteneur` : affiche le chemin du fichier journal d'un conteneur
- `docker inspect -f '\{\{.Config.Image\}\}' conteneur` : affiche le nom de l'image d'un conteneur
- `docker inspect -f '\{\{json .Config\}\}' conteneur` : affiche les informations de configuration en JSON
- `docker inspect -f '\{\{range $p, $conf := .NetworkSettings.Ports\}\} \{\{$p\}\} -> \{\{(index $conf 0).HostPort\}\} \{\{end\}\}' conteneur` : affiche toutes les liaisons de port
- `docker inspect` : affiche l'aide

## Links
- More information: https://docs.docker.com/reference/cli/docker/inspect/
