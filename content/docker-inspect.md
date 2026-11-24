# docker inspect

Return low-level information on Docker objects.

## Usage
- `docker inspect container|image|id` : display information about a container, image, or volume using a name or ID
- `docker inspect -f '\{\{range.NetworkSettings.Networks\}\}\{\{.IPAddress\}\}\{\{end\}\}' container` : display a container's IP address
- `docker inspect -f '\{\{.LogPath\}\}' container` : display the path to the container's log file
- `docker inspect -f '\{\{.Config.Image\}\}' container` : display the image name of the container
- `docker inspect -f '\{\{json .Config\}\}' container` : display the configuration information as JSON
- `docker inspect -f '\{\{range $p, $conf := .NetworkSettings.Ports\}\} \{\{$p\}\} -> \{\{(index $conf 0).HostPort\}\} \{\{end\}\}' container` : display all port bindings
- `docker inspect` : display help

## Categories
containers, desktop, media

## Links
- More information: https://docs.docker.com/reference/cli/docker/inspect/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-inspect
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
