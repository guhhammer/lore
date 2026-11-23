# docker compose

Executa e gerencia multi-containers de aplicações Docker.

## Usage
- `docker compose ps` : lista todos os containers em execução
- `docker compose up -d` : cria e inicia todos os containers em segundo plano usando um arquivo `docker-compose.yml` do seu diretório atual
- `docker compose up --build` : inicia todos os containers. Se necessário, realiza um rebuild
- `docker compose -p nome_do_projeto -f caminho/para/arquivo up` : inicia todos os containers especificando um nome de projeta e usando um arquivo de composição alternativo
- `docker compose stop` : encerra todos os containers em execução
- `docker compose down --rmi all -v` : encerra e remove todos os containers, networks, imagens e volumes
- `docker compose logs -f` : segue os logs de todos os containers
- `docker compose logs -f nome_container` : segue os logs de um container específico

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/
