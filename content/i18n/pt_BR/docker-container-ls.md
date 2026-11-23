# docker container ls

Lista os contêineres Docker.

## Usage
- `docker ps` : lista contêineres Docker em execução
- `docker ps -a` : lista todos contêineres Docker (em execução e parados)
- `docker ps -l` : lista os últimos contêineres criados (inclui todos os estados)
- `docker ps -f "name=nome"` : filtra os contêineres que contêm uma substring no seu nome
- `docker ps -f "ancestor=imagem:tag"` : filtra todos os contêineres que compartilham uma determinada imagem com um antepassado
- `docker ps -f "exited=código" -a` : filtra contêineres que tenham o código de saída
- `docker ps -f "status=estado"` : filtra contêineres por estado (created, running, removing, paused, exited e dead)
- `docker ps -f "volume=caminho/para/diretório" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : filtra contêineres que montem um volume específico ou tenham um volume montado em um caminho específico

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/ls/
