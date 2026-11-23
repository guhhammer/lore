# docker secret

Gerenciar segredos do Docker swarm.

## Usage
- `comando | docker secret create nome_do_segredo -` : cria um novo segredo a partir de `stdin`
- `docker secret create nome_do_segredo caminho/para/arquivo` : cria um novo segredo a partir de um arquivo
- `docker secret ls` : lista todos os segredos
- `docker secret inspect --pretty nome_do_segredo1 nome_do_segredo2 ...` : exibe informações detalhadas sobre um ou vários segredos em um formato amigável ao usuário
- `docker secret rm nome_do_segredo1 nome_do_segredo2 ...` : remove um ou mais segredos

## Links
- More information: https://docs.docker.com/reference/cli/docker/secret/
