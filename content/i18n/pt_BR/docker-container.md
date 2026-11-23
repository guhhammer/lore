# docker container

Gerenciar contêineres Docker.

## Usage
- `docker ps` : lista os contêineres Docker em execução
- `docker start nome_do_contêiner1 nome_do_contêiner2 ...` : inicia um ou mais contêineres parados
- `docker kill nome_do_contêiner1 nome_do_contêiner2 ...` : encerra um ou mais contêineres em execução
- `docker stop nome_do_contêiner1 nome_do_contêiner2 ...` : para um ou mais contêineres em execução
- `docker pause nome_do_contêiner1 nome_do_contêiner2 ...` : pausa todos os processos em um ou mais contêineres
- `docker container inspect nome_do_contêiner1 nome_do_contêiner2 ...` : exibe informações detalhadas sobre um ou mais contêineres
- `docker export nome_do_contêiner` : exporta o sistema de arquivos de um contêiner como um arquivo `.tar`
- `docker commit nome_do_contêiner` : cria uma nova imagem a partir das alterações em um contêiner

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/
