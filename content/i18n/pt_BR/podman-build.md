# podman build

Ferramenta sem daemon para criar imagens de contêiner.

## Usage
- `podman build caminho/para/diretório` : cria uma imagem usando um `Dockerfile` ou `Containerfile` no diretório especificado
- `podman build -t nome_da_imagem:versão caminho/para/diretório` : cria uma imagem com uma tag especificada
- `podman build -f Containerfile.diferente .` : cria uma imagem a partir de um arquivo não padrão
- `podman build --no-cache caminho/para/diretório` : cria uma imagem sem usar nenhuma imagem em cache previamente
- `podman build -q caminho/para/diretório` : cria uma imagem suprimindo todas as saídas

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-build.1.html
