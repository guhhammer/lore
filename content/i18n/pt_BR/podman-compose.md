# podman-compose

Executar e gerenciar definição de contêineres Compose Specification.

## Usage
- `podman-compose ps` : lista todos os contêineres em execução
- `podman-compose up -d` : cria e inicia todos os contêineres em segundo plano usando um arquivo `docker-compose.yml` local
- `podman-compose up --build` : inicia todos os contêineres, fazendo o build se necessário
- `podman-compose -f caminho/para/arquivo up` : inicia todos os contêineres usando um arquivo de composição alternativo
- `podman-compose stop` : para todos os contêineres em execução
- `podman-compose down -v` : remove todos os contêineres, redes e volumes
- `podman-compose logs -f nome_do_contêiner` : acompanha logs de um contêiner (omite todos os nomes de contêineres)
- `podman-compose run nome_do_serviço comando` : executa um comando único em um serviço sem mapear portas

## Links
- More information: https://github.com/containers/podman-compose
