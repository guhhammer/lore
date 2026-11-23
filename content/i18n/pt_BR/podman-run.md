# podman run

Executar um comando em um novo contêiner Podman.

## Usage
- `podman run imagem:tag comando` : executa um comando em um novo contêiner a partir de uma imagem marcada
- `podman run -d imagem:tag comando` : executa um comando em um novo contêiner em segundo plano e exibe o ID
- `podman run --rm -it imagem:tag comando` : executa um comando em um contêiner temporário no modo interativo e pseudo-TTY
- `podman run -e 'variável=valor' -e variável imagem:tag comando` : executa um comando em um novo contêiner com variáveis de ambiente passadas
- `podman run -v /caminho/para/caminho_no_host:/caminho/para/caminho_no_contêiner imagem:tag comando` : executa um comando em um novo contêiner com volumes montados por bind
- `podman run -p porta_no_host:porta_no_contêiner imagem:tag comando` : executa um comando em um novo contêiner com portas publicadas
- `podman run --entrypoint comando imagem:tag` : executa um comando em um novo contêiner sobrescrevendo o ponto de entrada (entrypoint) da imagem
- `podman run --network rede imagem:tag` : executa um comando em um novo contêiner conectando-o a uma rede

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-run.1.html
