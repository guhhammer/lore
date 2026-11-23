# docker

Gerenciador de containers e imagens Docker. Alguns subcomandos como `run` tem sua própia documentação de uso.

## Usage
- `docker ps -a` : lista todos os containers Docker (em execução e parados)
- `docker run --name nome_container imagem` : inicializa um container com um nome personalizado a partir de uma imagem
- `docker container start|stop nome_container` : começa ou para um container existente
- `docker pull imagem` : extrai uma imagem a partir de um Docker Registry
- `docker images` : mostra a lista de imagens já baixadas
- `docker exec -it nome_container sh` : abre um terminal dentro de um container em execução
- `docker rm nome_container` : remove um container parado
- `docker logs -f nome_container` : obtém e acompanha o histórico de um container

## Links
- More information: https://docs.docker.com/reference/cli/docker/
