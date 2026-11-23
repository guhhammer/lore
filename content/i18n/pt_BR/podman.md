# podman

Ferramenta de gerenciamento simples para pods, contêineres e imagens. O Podman fornece uma linha de comando comparável ao Docker-CLI. Simplificando: `alias docker=podman`.

## Usage
- `podman ps -a` : lista todos os contêineres (em execução e parados)
- `podman run --name nome_do_contêiner imagem` : cria um contêiner a partir de uma imagem, com um nome personalizado
- `podman start|stop nome_do_contêiner` : inicia ou para um contêiner existente
- `podman pull imagem` : baixa uma imagem de um registro (por padrão, Docker Hub)
- `podman images` : exibe a lista de imagens já baixadas
- `podman exec -it nome_do_contêiner sh` : abre um shell dentro de um contêiner que já está em execução
- `podman rm nome_do_contêiner` : remove um contêiner parado
- `podman logs -f nome_do_contêiner id_do_contêiner ...` : exibe os logs de um ou mais contêineres e acompanha a saída do log

## Links
- More information: https://docs.podman.io/en/latest/Commands.html
