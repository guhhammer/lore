# podman images

Gerenciar imagens de contêineres OCI/Docker.

## Usage
- `podman images` : lista todas as imagens de contêineres
- `podman images -a` : lista todas as imagens de contêiner, incluindo intermediárias
- `podman images -q` : lista a saída no modo silencioso (apenas IDs numéricos)
- `podman images -f dangling=true` : lista todas as imagens de contêiner que não são utilizadas por nenhum contêiner
- `podman images "*imagem|imagem*"` : lista imagens que contenham uma substring em seus nomes

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-images.1.html
