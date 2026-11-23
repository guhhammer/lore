# docker container exec

Executar um comando em um contêiner Docker em execução.

## Usage
- `docker exec -it nome_do_contêiner /bin/bash` : entra em uma sessão de shell interativa em um contêiner em execução
- `docker exec -d nome_do_contêiner comando` : executa um comando em segundo plano (detached) em um contêiner em execução
- `docker exec -it -w caminho/para/diretório nome_do_contêiner comando` : seleciona o diretório de trabalho para a execução de um determinado comando
- `docker exec -i -d nome_do_contêiner comando` : executa um comando em segundo plano em um contêiner existente, mas mantém o `stdin` aberto
- `docker exec -it -e nome_da_variável=valor nome_do_contêiner /bin/bash` : define uma variável de ambiente em uma sessão Bash em execução
- `docker exec -u usuário nome_do_contêiner comando` : executa um comando como um usuário específico

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/exec/
