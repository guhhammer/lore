# docker compose up

Start and run Docker services defined in a Compose file.

## Usage
- `docker compose up` : start all services defined in the docker-compose file
- `docker compose up -d` : start services in the background (detached mode)
- `docker compose up --build` : start services and rebuild images before starting
- `docker compose up service1 service2 ...` : start specific services only
- `docker compose -f path/to/config up` : start services with custom compose file
- `docker compose up --remove-orphans` : start services and remove orphaned containers
- `docker compose up --scale service=count` : start services with scaled instances
- `docker compose up --timestamps` : start services and show logs with timestamps

## Categories
containers, system, files

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/up/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-compose-up
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
