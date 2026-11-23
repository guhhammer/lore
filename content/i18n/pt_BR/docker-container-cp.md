# docker container cp

Copia arquivos ou diretórios entre filesystems do host e container.

## Usage
- `docker cp caminho/do/arquivo_ou_diretório_do_host nome_do_container:caminho/do/arquivo_ou_diretório_do_container` : copia um arquivo ou diretório de um host para um container
- `docker cp nome_do_container:caminho/do/arquivo_ou_diretório_do_container caminho/do/arquivo_ou_diretório_do_host` : copia um arquivo ou diretório de um container para o host
- `docker cp -L caminho/do/arquivo_ou_diretório_linkado_do_host nome_do_container:caminho/do/arquivo_ou_diretório_do_container` : copia um diretório de um host para um container, seguindo o link simbólico (copia os arquivos linkados diretamente e não o link simbólico)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/cp/
