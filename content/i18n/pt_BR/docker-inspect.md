# docker inspect

Retorna informações de baixo nível sobre objetos do Docker.

## Usage
- `docker inspect contêiner|imagem|id` : exibe informações sobre um contêiner, imagem ou volume usando um nome ou ID
- `docker inspect -f '\{\{range.NetworkSettings.Networks\}\}\{\{.IPAddress\}\}\{\{end\}\}' contêiner` : exibe o endereço IP de um contêiner
- `docker inspect -f '\{\{.LogPath\}\}' contêiner` : exibe o caminho para o arquivo de log do contêiner
- `docker inspect -f '\{\{.Config.Image\}\}' contêiner` : exibe o nome da imagem do contêiner
- `docker inspect -f '\{\{json .Config\}\}' contêiner` : exibe as informações de configuração como JSON
- `docker inspect -f '\{\{range $p, $conf := .NetworkSettings.Ports\}\} \{\{$p\}\} -> \{\{(index $conf 0).HostPort\}\} \{\{end\}\}' contêiner` : exibe todas as portas vinculadas
- `docker inspect` : mostra ajuda

## Links
- More information: https://docs.docker.com/reference/cli/docker/inspect/
