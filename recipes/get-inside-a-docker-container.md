# get inside a docker container

Open a shell in a running container, copy files in and out, read its logs, share a folder with it.

## Steps
- `docker ps` : the running containers and their names
- `docker exec -it NAME sh` : a shell inside it (bash, if the image has it)
- `docker logs -f NAME` : its output, as it comes
- `docker cp NAME:/path/file ./` : copy a file out of it
- `docker run -it --rm -v "$PWD":/work -w /work ubuntu bash` : a new container with this folder inside it
- `docker compose exec SERVICE sh` : with Compose, by service name

## See also
- docker

## Categories
containers
