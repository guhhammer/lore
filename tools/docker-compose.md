# docker compose

Docker Compose describes several containers (app, database, cache) in one compose.yaml file and starts them together. Use `docker compose` (with a space) on current Docker.

## Tasks
### Start and stop
- `docker compose up -d` : create and start everything in the background
- `docker compose up -d --build` : rebuild images first
- `docker compose up -d SERVICE` : start only one service and what it needs
- `docker compose stop` : stop without removing
- `docker compose start` : start stopped services again
- `docker compose restart SERVICE` : restart one service
- `docker compose down` : stop and remove containers and networks
- `docker compose down -v` : also delete the volumes (deletes data)
### Look
- `docker compose ps` : the project's containers
- `docker compose logs -f SERVICE` : follow a service's logs
- `docker compose logs --tail 50` : the last lines of every service
- `docker compose top` : processes inside each container
- `docker compose config` : print the file as Compose reads it, to catch mistakes
- `docker compose ls` : projects running on this machine
### Work inside
- `docker compose exec SERVICE bash` : shell in a running service
- `docker compose run --rm SERVICE CMD` : run a one-off command in a new container
- `docker compose cp SERVICE:/path/file .` : copy a file out
### Images
- `docker compose build` : build the images
- `docker compose build --no-cache SERVICE` : rebuild one from scratch
- `docker compose pull` : download newer images
- `docker compose up -d --pull always` : pull then start
- `docker compose images` : images in use
### Settings
- `docker compose -f FILE.yaml up -d` : use another compose file
- `docker compose --env-file FILE.env up -d` : use another environment file
- `docker compose -p NAME up -d` : choose the project name
- `docker compose --profile NAME up -d` : start the services of a profile

## Notes
- The old command was `docker-compose` (with a hyphen); the plugin `docker compose` replaced it.
- `down -v` deletes named volumes, which usually means the database.
- `depends_on` orders start-up but does not wait for a database to be ready: add a healthcheck.
- Keep secrets in an `.env` file out of git, not inside compose.yaml.

## See also
- docker
- podman
- kubectl

## Links
- CLI: https://docs.docker.com/reference/cli/docker/compose/
- File reference: https://docs.docker.com/reference/compose-file/

## Categories
containers
