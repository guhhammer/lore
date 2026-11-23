# docker inspect

Erhalte tiefgehende Informationen zu Docker Objekten.

## Usage
- `docker inspect container|image|id` : zeige Informationen über einen Container, ein Image oder Volume anhand des Namens oder der ID
- `docker inspect -f '\{\{range.NetworkSettings.Networks\}\}\{\{.IPAddress\}\}\{\{end\}\}' container` : zeige die IP Adresse eines Containers an
- `docker inspect -f '\{\{.LogPath\}\}' container` : zeige den Pfad zur Logdatei eines Containers
- `docker inspect -f '\{\{.Config.Image\}\}' container` : zeige den Namen des Images eines Containers
- `docker inspect -f '\{\{json .Config\}\}' container` : zeige die Konfiguration als JSON an
- `docker inspect -f '\{\{range $p, $conf := .NetworkSettings.Ports\}\} \{\{$p\}\} -> \{\{(index $conf 0).HostPort\}\} \{\{end\}\}' container` : zeige alle Port Bindings
- `docker inspect` : zeige Hilfe an

## Links
- More information: https://docs.docker.com/reference/cli/docker/inspect/
