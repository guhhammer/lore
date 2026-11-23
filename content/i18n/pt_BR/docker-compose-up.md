# docker compose up

Inicia e executa os serviços Docker definidos em um arquivo Compose.

## Usage
- `docker compose up` : inicia todos os serviços definidos no arquivo docker-compose
- `docker compose up -d` : inicia os serviços em segundo plano (modo desanexado)
- `docker compose up --build` : inicia os serviços e reconstrói as imagens antes de iniciar
- `docker compose up serviço1 serviço2 ...` : inicia apenas serviços específicos
- `docker compose -f caminho/para/config up` : inicia os serviços com um arquivo compose personalizado
- `docker compose up --remove-orphans` : inicia os serviços e remove contêineres órfãos
- `docker compose up --scale serviço=contagem` : inicia os serviços com instâncias em escala
- `docker compose up --timestamps` : inicia os serviços e mostra os registros com timestamps

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/up/
