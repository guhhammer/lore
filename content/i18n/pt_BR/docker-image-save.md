# docker image save

Exportar uma ou mais imagens do Docker para um arquivo de arquivamento.

## Usage
- `docker save imagem:tag >caminho/para/arquivo.tar` : salva uma imagem redirecionando `stdout` para um arquivo `.tar`
- `docker save -o caminho/para/arquivo.tar imagem:tag` : salva uma imagem em um arquivo `.tar`
- `docker save -o caminho/para/arquivo.tar nome_da_imagem` : salva todas as tags da imagem
- `docker save -o caminho/para/arquivo.tar nome_da_imagem:tag1 nome_da_imagem:tag2 ...` : seleciona tags específicas de uma imagem para salvar

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/save/
