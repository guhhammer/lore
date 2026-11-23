# podman rmi

Remover uma ou mais imagens do OCI/Docker.

## Usage
- `podman rmi imagem:tag imagem2:tag ...` : remove uma ou mais imagens pelo nome delas
- `podman rmi -f imagem` : remove uma imagem forçadamente
- `podman rmi --no-prune imagem` : remove uma imagem sem excluir os pais não marcados
- `podman rmi -h` : mostra ajuda

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-rmi.1.html
